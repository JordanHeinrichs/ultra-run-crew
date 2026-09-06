use std::vec;

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use axum_session_sqlx::SessionSqlitePool;
use serde::{Deserialize, Serialize};

use crate::{AppState, errors::AppError};

type Session = axum_session::Session<SessionSqlitePool>;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(races_list))
}

#[derive(Serialize)]
pub struct Race {}

#[derive(Serialize)]
pub struct RacesListResponse {
    pub races: Vec<Race>,
}

async fn races_list(
    State(state): State<AppState>,
    session: Session,
) -> Result<Json<RacesListResponse>, AppError> {
    Ok(Json(RacesListResponse { races: vec![] }))
}
