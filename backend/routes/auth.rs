use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use axum_session_sqlx::SessionSqlitePool;
use serde::{Deserialize, Serialize};

use crate::{AppState, errors::AppError};

type Session = axum_session::Session<SessionSqlitePool>;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub name: String,
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
        .route("/register", post(register))
        .route("/me", get(me))
}

// --- Handlers ---

/// POST /api/auth/login
async fn login(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = sqlx::query!(
        "SELECT id, email, password_hash FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| AppError::LoginError())?
    .ok_or(AppError::LoginError())?;

    let raw_hash = user.password_hash.ok_or(AppError::LoginError())?;
    let password_hash = PasswordHash::new(&raw_hash)?;

    Argon2::default().verify_password(&payload.password.as_bytes(), &password_hash)?;

    session.set("user_id", user.id);
    session.set("email", user.email.clone());

    Ok(Json(UserResponse {
        id: user.id,
        email: user.email,
    }))
}

/// POST /api/auth/register
async fn register(
    State(state): State<AppState>,
    session: Session,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes())?
        .to_string();

    let result = sqlx::query!(
        "INSERT INTO users (name, email, password_hash, provider) VALUES (?, ?, ?, ?)",
        payload.name,
        payload.email,
        password_hash,
        "username-password"
    )
    .execute(&state.db)
    .await?;

    let id = result.last_insert_rowid();

    session.set("user_id", id);
    session.set("email", payload.email.clone());

    Ok(Json(UserResponse {
        id,
        email: payload.email,
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
