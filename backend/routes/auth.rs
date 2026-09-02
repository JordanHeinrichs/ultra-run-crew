use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_session_sqlx::SessionSqlitePool;
use serde::{Deserialize, Serialize};

use crate::AppState;

type Session = axum_session::Session<SessionSqlitePool>;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
}

// --- Auth Router Setup ---

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

// --- Handlers ---

/// POST /auth/login
async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, (StatusCode, &'static str)> {
    // 1. Fetch user from SQLite using state.db
    // (Adjust column names to match your schema)
    let user = sqlx::query!(
        "SELECT id, email, password_hash FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error"))?;

    let user = match user {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid username or password")),
    };

    // 2. Verify password (e.g., using argon2 or bcrypt)
    // let valid = verify_password(&payload.password, &user.password_hash);
    // if !valid { return Err((StatusCode::UNAUTHORIZED, "Invalid credentials")); }

    // 3. Store user identity in the session
    session.set("user_id", user.id);
    session.set("email", user.email.clone());

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
    }))
}

/// POST /auth/logout
async fn logout(session: Session) -> (StatusCode, &'static str) {
    // Destroys the session in memory and marks it for deletion in SQLite
    session.destroy();
    (StatusCode::OK, "Logged out successfully")
}

/// GET /auth/me
async fn me(session: Session) -> Result<Json<UserResponse>, StatusCode> {
    // Synchronous reads from in-memory session cache
    let user_id: i64 = session.get("user_id").ok_or(StatusCode::UNAUTHORIZED)?;
    let email: String = session.get("email").ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(Json(UserResponse { id: user_id, email }))
}
