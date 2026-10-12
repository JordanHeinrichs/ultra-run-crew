use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    routing::{get, post},
};
use sqlx::{QueryBuilder, Sqlite};

use crate::{
    AppState,
    db::{
        aid_station_repository::AidStationRepository, course_repository::CourseRepository,
        models::Course,
    },
    errors::AppError::{self, BadRequest, InternalServerError},
    helpers::predictor::{CoursePredictions, calculate_predictions},
};
use crate::{auth_middleware::AuthenticatedUser, db::sqlite_repository::SqliteRepository};
use crate::{db::race_repository::RaceRepository, helpers::gpx_parser::generate_segments_from_gpx};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{race_id}/upload", post(course_upload))
        .route(
            "/{race_id}/{course_id}/predict",
            get(get_course_predictions),
        )
}

async fn course_upload(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(race_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Course>, AppError> {
    let repository = SqliteRepository::new(state.db.clone(), user.clone());
    let _race = repository.find_race(race_id).await?;

    let mut file_bytes: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| BadRequest("Failed to get multipart file".into()))?
    {
        if field.name() == Some("file") {
            file_name = field.file_name().map(|s| s.to_string());
            let bytes = field.bytes().await.map_err(|_| InternalServerError())?;

            file_bytes = Some(bytes.to_vec());
            break; // Stop parsing after finding the file
        }
    }

    println!(
        "Processing GPX for race_id: {}, filename: {:?}",
        race_id, file_name
    );

    let bytes = file_bytes.ok_or(BadRequest("Failed to get multipart file".into()))?;
    let segments = generate_segments_from_gpx(bytes)?;
    if segments.len() == 0 {
        return Err(BadRequest("Unable to parse distance from GPX".into()));
    }

    let course = sqlx::query_as!(
        Course,
        r#"
        INSERT INTO courses (race_id, total_distance_km, total_elevation_gain_m, total_elevation_loss_m, is_deleted, created_at, updated_at)
        VALUES (?, ?, ?, ?, FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id as "id!", race_id as "race_id!", total_distance_km as "total_distance_km!", total_elevation_gain_m as "total_elevation_gain_m!",  total_elevation_loss_m as "total_elevation_loss_m!", is_deleted as "is_deleted!", created_at as "created_at!", updated_at as "updated_at!"
        "#,
        race_id,
        segments.last().unwrap().km,
        segments.iter().map(|s| s.gain_m).sum::<f64>(),
        segments.iter().map(|s| s.loss_m).sum::<f64>(),
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::InternalServerError())?;

    let mut query_builder: QueryBuilder<Sqlite> =
        QueryBuilder::new("INSERT INTO course_segments (course_id, km, gain_m, loss_m) ");

    query_builder.push_values(segments, |mut b, segment| {
        b.push_bind(course.id)
            .push_bind(segment.km)
            .push_bind(segment.gain_m)
            .push_bind(segment.loss_m);
    });

    let query = query_builder.build();
    query.execute(&state.db).await?;

    Ok(Json(course))
}

async fn get_course_predictions(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((race_id, course_id)): Path<(i64, i64)>,
) -> Result<Json<Vec<CoursePredictions>>, AppError> {
    let repository = SqliteRepository::new(state.db, user);
    let _race = repository.find_race(race_id).await?;
    let aid_stations = repository.list_aid_stations(race_id).await?;
    let course = repository.find_course(course_id).await?;
    let predictions = calculate_predictions(&course, &aid_stations)?;
    Ok(Json(predictions))
}
