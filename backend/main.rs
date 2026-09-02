use axum::Router;
use axum_csrf::CsrfConfig;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionSqlitePool;
use rust_embed::Embed;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod errors;
mod routes;

#[allow(dead_code)]
#[derive(Embed, Clone)]
#[folder = "build/"]
struct Assets;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub csrf_config: CsrfConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    tracing::info!("Environment variables loaded successfully.");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL variable must be set in .env");
    let db_pool = sqlx::SqlitePool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Database connected and migrations applied.");

    let session_config = SessionConfig::default().with_table_name("sessions");
    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(db_pool.clone().into()), session_config)
            .await?;

    // let is_production = std::env::var("APP_ENV").unwrap_or_default() == "production";

    // 6. Initialize CSRF Configuration Layer
    // Generates a new random cryptographic signature key on every boot sequence
    let csrf_config = CsrfConfig::default();

    // 7. Assemble Global Application State
    let state = AppState {
        db: db_pool,
        csrf_config,
    };

    // 8. Build the Router Architecture and Chain Middleware
    let app = Router::new()
        // Mount sub-routers from your routes/ directory module
        // .nest("/", routes::dashboard::router())
        .nest("/api/auth", routes::auth::router())
        .fallback_service(
            ServeDir::new("build").not_found_service(ServeFile::new("build/app.html")),
        )
        .layer(TraceLayer::new_for_http())
        .layer(SessionLayer::new(session_store))
        .with_state(state);

    // 9. Bind Network Port and Boot Server Instance
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("🚀 Application running successfully at http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
