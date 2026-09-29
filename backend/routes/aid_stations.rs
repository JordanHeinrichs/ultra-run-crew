use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use serde::Deserialize;

use crate::{AppState, errors::AppError};
use crate::{auth_middleware::AuthenticatedUser, db_models::AidStation};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(aid_stations_list).post(aid_station_create))
        .route(
            "/{id}",
            get(aid_station_get)
                .put(aid_station_edit)
                .delete(aid_station_delete),
        )
}

#[derive(Deserialize)]
pub struct CreateAidStationPayload {
    pub race_id: i64,
    pub name: String,
    pub distance_km: f64,
    pub is_crew_allowed: Option<bool>,
    pub has_drop_bag: Option<bool>,
    pub arrival_timestamp: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateAidStationPayload {
    pub name: Option<String>,
    pub distance_km: Option<f64>,
    pub is_crew_allowed: Option<bool>,
    pub has_drop_bag: Option<bool>,
    pub arrival_timestamp: Option<i64>,
}

async fn aid_stations_list(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<AidStation>>, AppError> {
    let aid_stations = sqlx::query_as!(
        AidStation,
        r#"
        SELECT a.id, a.race_id, a.name, a.distance_km, a.is_crew_allowed, a.has_drop_bag, a.is_deleted, a.arrival_timestamp, a.created_at, a.updated_at
        FROM aid_stations a
        JOIN races r ON a.race_id = r.id
        WHERE r.runner_id = ? AND a.is_deleted = FALSE
        ORDER BY a.distance_km ASC
        "#,
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(aid_stations))
}

async fn aid_station_create(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateAidStationPayload>,
) -> Result<Json<AidStation>, AppError> {
    let aid_station = sqlx::query_as!(
        AidStation,
        r#"
        INSERT INTO aid_stations (race_id, name, distance_km, is_crew_allowed, has_drop_bag, is_deleted, arrival_timestamp, created_at, updated_at)
        SELECT r.id, ?, ?, COALESCE(?, FALSE), COALESCE(?, FALSE), FALSE, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
        FROM races r
        WHERE r.id = ? AND r.runner_id = ? AND r.is_deleted = FALSE
        RETURNING id as "id!", race_id as "race_id!", name as "name!", distance_km as "distance_km!", is_crew_allowed as "is_crew_allowed!", has_drop_bag as "has_drop_bag!", is_deleted as "is_deleted!", arrival_timestamp as "arrival_timestamp!", created_at as "created_at!", updated_at as "updated_at!"
        "#,
        payload.name,
        payload.distance_km,
        payload.is_crew_allowed,
        payload.has_drop_bag,
        payload.arrival_timestamp,
        payload.race_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found".into()))?;

    Ok(Json(aid_station))
}

async fn aid_station_get(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<AidStation>, AppError> {
    let aid_station = sqlx::query_as!(
        AidStation,
        r#"
        SELECT a.id, a.race_id, a.name, a.distance_km, a.is_crew_allowed, a.has_drop_bag, a.is_deleted, a.arrival_timestamp, a.created_at, a.updated_at
        FROM aid_stations a
        JOIN races r ON a.race_id = r.id
        WHERE a.id = ? AND r.runner_id = ? AND a.is_deleted = FALSE
        "#,
        id,
        user.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(aid_station))
}

async fn aid_station_edit(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateAidStationPayload>,
) -> Result<Json<AidStation>, AppError> {
    let aid_station = sqlx::query_as!(
        AidStation,
        r#"
        UPDATE aid_stations
        SET
            name = COALESCE(?, name),
            distance_km = COALESCE(?, distance_km),
            is_crew_allowed = COALESCE(?, is_crew_allowed),
            has_drop_bag = COALESCE(?, has_drop_bag),
            arrival_timestamp = COALESCE(?, arrival_timestamp),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND race_id IN (SELECT id FROM races WHERE runner_id = ? AND is_deleted = FALSE) AND is_deleted = FALSE
        RETURNING id, race_id, name, distance_km, is_crew_allowed, has_drop_bag, is_deleted, arrival_timestamp, created_at, updated_at
        "#,
        payload.name,
        payload.distance_km,
        payload.is_crew_allowed,
        payload.has_drop_bag,
        payload.arrival_timestamp,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Aid station not found or already deleted".into()))?;

    Ok(Json(aid_station))
}

async fn aid_station_delete(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<AidStation>, AppError> {
    let aid_station = sqlx::query_as!(
        AidStation,
        r#"
        UPDATE aid_stations
        SET is_deleted = TRUE, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND race_id IN (SELECT id FROM races WHERE runner_id = ? AND is_deleted = FALSE) AND is_deleted = FALSE
        RETURNING id, race_id, name, distance_km, is_crew_allowed, has_drop_bag, is_deleted, arrival_timestamp, created_at, updated_at
        "#,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Aid station not found".into()))?;

    Ok(Json(aid_station))
}
