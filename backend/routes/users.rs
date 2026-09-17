use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{AppState, errors::AppError};
use crate::{auth_middleware::AuthenticatedUser, db::User};

pub fn router() -> Router<AppState> {
    Router::new().route("/{id}", get(user_get))
}

async fn user_get(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<User>, AppError> {
    if (user.user_id != id) {
        // In the future users should be able to get other users on their team. For now only self.
        return Err(AppError::NotFound("User not found".into()));
    }
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, name, provider, is_active, '' as password_hash, created_at, updated_at
        FROM users
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(user))
}
