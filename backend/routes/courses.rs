use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    routing::post,
};
use serde::Deserialize;

use crate::{
    AppState,
    db_models::Course,
    errors::AppError::{self, BadRequest, InternalServerError, NotFound},
};
use crate::{auth_middleware::AuthenticatedUser, db_models::AidStation};

pub fn router() -> Router<AppState> {
    Router::new().route("/{race_id}/upload", post(course_upload))
}

async fn course_upload(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(race_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Course>, AppError> {
    let has_race_access = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM races r
            WHERE r.id = ? AND r.runner_id = ? AND r.is_deleted = FALSE
        ) as "exists!: bool"
        "#,
        race_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found".into()))?;

    if !has_race_access {
        return Err(NotFound("Race not found".into()));
    }

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

    let bytes = file_bytes.ok_or(BadRequest("Failed to get multipart file".into()))?;

    println!(
        "Processing GPX for race_id: {}, filename: {:?}",
        race_id, file_name
    );

    // TODO: Do the GPX processing function and produce the following information and save all the segments

    let course = sqlx::query_as!(
        Course,
        r#"
        INSERT INTO courses (race_id, total_distance_km, total_elevation_gain_m, total_elevation_loss_m, is_deleted, created_at, updated_at)
        VALUES (?, ?, ?, ?, FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id as "id!", race_id as "race_id!", total_distance_km as "total_distance_km!", total_elevation_gain_m as "total_elevation_gain_m!",  total_elevation_loss_m as "total_elevation_loss_m!", is_deleted as "is_deleted!", created_at as "created_at!", updated_at as "updated_at!"
        "#,
        race_id,
        0,
        0,
        0,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::InternalServerError())?;

    Ok(Json(course))
}
