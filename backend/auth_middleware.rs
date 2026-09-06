use crate::AppState;
use axum::{
    body::Body, extract::State, http::Request, http::StatusCode, middleware::Next,
    response::Response,
};
use axum_session::ReadOnlySession;
use axum_session_sqlx::SessionSqlitePool;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub email: String,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    session: ReadOnlySession<SessionSqlitePool>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id: i64 = session.get("user_id").ok_or(StatusCode::UNAUTHORIZED)?;

    let user = sqlx::query_as!(
        AuthenticatedUser,
        "SELECT id as user_id, email FROM users WHERE id = ?",
        user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
