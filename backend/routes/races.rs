use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{AppState, errors::AppError};
use crate::{auth_middleware::AuthenticatedUser, db_models::Race};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(races_list).post(race_create))
        .route("/{id}", get(race_get).put(race_edit).delete(race_delete))
}

#[derive(Serialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct RaceListRace {
    pub id: i64,
    pub runner_id: i64,
    pub name: String,
    #[ts(type = "string")]
    pub event_date: NaiveDate,
    pub distance_km: f64,
    pub goal_duration: Option<f64>,
    pub is_deleted: bool,
    pub runner_name: String,
}
pub type RacesListResponse = Vec<RaceListRace>;

#[derive(Deserialize)]
pub struct CreateRacePayload {
    pub name: String,
    pub event_date: NaiveDate,
    pub distance_km: f64,
    pub goal_duration: Option<f64>,
    pub pacing_strategy: Option<f64>,
    pub default_aid_station_duration: Option<f64>,
}

#[derive(Deserialize)]
pub struct UpdateRacePayload {
    pub name: Option<String>,
    pub event_date: Option<NaiveDate>,
    pub distance_km: Option<f64>,
    pub goal_duration: Option<f64>,
    pub pacing_strategy: Option<f64>,
    pub default_aid_station_duration: Option<f64>,
}

async fn races_list(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<RacesListResponse>, AppError> {
    let races = sqlx::query_as!(
        RaceListRace,
        r#"
        SELECT r.id, r.runner_id, r.name, u.name as runner_name, r.event_date, r.is_deleted, r.distance_km, r.goal_duration
        FROM races r
        JOIN users u ON r.runner_id = u.id
        WHERE r.runner_id = ? AND r.is_deleted = FALSE
        ORDER BY r.event_date ASC
        "#,
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(races))
}

async fn race_create(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRacePayload>,
) -> Result<Json<Race>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        INSERT INTO races (runner_id, name, event_date, distance_km, goal_duration, pacing_strategy, default_aid_station_duration, is_deleted, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id as "id!", runner_id as "runner_id!", name as "name!", event_date as "event_date!", is_deleted as "is_deleted!", created_at as "created_at!", updated_at as "updated_at!", distance_km as "distance_km!", goal_duration as "goal_duration!", pacing_strategy as "pacing_strategy!", default_aid_station_duration as "default_aid_station_duration!"
        "#,
        user.user_id,
        payload.name,
        payload.event_date,
        payload.distance_km,
        payload.goal_duration,
        payload.pacing_strategy,
        payload.default_aid_station_duration
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(race))
}

async fn race_get(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Race>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        SELECT id, runner_id, name, event_date, is_deleted, distance_km, goal_duration, pacing_strategy, default_aid_station_duration, created_at, updated_at
        FROM races
        WHERE id = ? AND runner_id = ? AND is_deleted = FALSE
        "#,
        id,
        user.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(race))
}

async fn race_edit(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRacePayload>,
) -> Result<Json<Race>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        UPDATE races
        SET
            name = COALESCE(?, name),
            event_date = COALESCE(?, event_date),
            distance_km = COALESCE(?, distance_km),
            goal_duration = COALESCE(?, goal_duration),
            pacing_strategy = COALESCE(?, pacing_strategy),
            default_aid_station_duration = COALESCE(?, default_aid_station_duration),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND runner_id = ? AND is_deleted = FALSE
        RETURNING id, runner_id, name, event_date, is_deleted, distance_km, goal_duration, pacing_strategy, default_aid_station_duration, created_at, updated_at
        "#,
        payload.name,
        payload.event_date,
        payload.distance_km,
        payload.goal_duration,
        payload.pacing_strategy,
        payload.default_aid_station_duration,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found or already deleted".into()))?;

    Ok(Json(race))
}

async fn race_delete(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Race>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        UPDATE races
        SET is_deleted = TRUE, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND runner_id = ? AND is_deleted = FALSE
        RETURNING id, runner_id, name, event_date, is_deleted, distance_km, goal_duration, pacing_strategy, default_aid_station_duration, created_at, updated_at
        "#,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found".into()))?;

    Ok(Json(race))
}
