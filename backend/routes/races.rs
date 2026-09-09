use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{AppState, errors::AppError};
use crate::{auth_middleware::AuthenticatedUser, db::Race};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(races_list).post(race_create))
        .route("/{id}", get(race_get).put(race_edit).delete(race_delete))
}

#[derive(Serialize, sqlx::FromRow, TS)]
#[ts(export)]
pub struct RaceListRace {
    pub id: i64,
    pub runner: i64,
    pub name: String,
    #[ts(type = "string")]
    pub event_date: NaiveDate,
    pub is_deleted: bool,
    pub runner_name: String,
}
pub type RacesListResponse = Vec<RaceListRace>;

#[derive(Serialize)]
pub struct RaceResponse {
    pub race: Race,
}

#[derive(Deserialize)]
pub struct CreateRacePayload {
    pub name: String,
    pub event_date: NaiveDate,
}

#[derive(Deserialize)]
pub struct UpdateRacePayload {
    pub name: Option<String>,
    pub event_date: Option<NaiveDate>,
}

async fn races_list(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<Json<RacesListResponse>, AppError> {
    let races = sqlx::query_as!(
        RaceListRace,
        r#"
        SELECT r.id, r.runner, r.name, u.name as runner_name, r.event_date, r.is_deleted
        FROM races r
        JOIN users u ON r.runner = u.id
        WHERE r.runner = ? AND r.is_deleted = FALSE
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
) -> Result<Json<RaceResponse>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        INSERT INTO races (runner, name, event_date, is_deleted, created_at, updated_at)
        VALUES (?, ?, ?, FALSE, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id as "id!", runner as "runner!", name as "name!", event_date as "event_date!", is_deleted as "is_deleted!", created_at as "created_at!", updated_at as "updated_at!"
        "#,
        user.user_id,
        payload.name,
        payload.event_date,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(RaceResponse { race }))
}

async fn race_get(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<RaceResponse>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        SELECT id, runner, name, event_date, is_deleted, created_at, updated_at
        FROM races
        WHERE id = ? AND runner = ? AND is_deleted = FALSE
        "#,
        id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found".into()))?;

    Ok(Json(RaceResponse { race }))
}

async fn race_edit(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRacePayload>,
) -> Result<Json<RaceResponse>, AppError> {
    let race = sqlx::query_as!(
        Race,
        r#"
        UPDATE races
        SET
            name = COALESCE(?, name),
            event_date = COALESCE(?, event_date),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND runner = ? AND is_deleted = FALSE
        RETURNING id, runner, name, event_date, is_deleted, created_at, updated_at
        "#,
        payload.name,
        payload.event_date,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found or already deleted".into()))?;

    Ok(Json(RaceResponse { race }))
}

async fn race_delete(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<RaceResponse>, AppError> {
    // Perform soft delete using the is_deleted flag
    let race = sqlx::query_as!(
        Race,
        r#"
        UPDATE races
        SET is_deleted = TRUE, updated_at = CURRENT_TIMESTAMP
        WHERE id = ? AND runner = ? AND is_deleted = FALSE
        RETURNING id, runner, name, event_date, is_deleted, created_at, updated_at
        "#,
        id,
        user.user_id,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Race not found".into()))?;

    Ok(Json(RaceResponse { race }))
}
