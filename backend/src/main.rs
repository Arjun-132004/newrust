use axum::{
    middleware as axum_middleware,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod config;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use config::database::DatabaseConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let db_config = DatabaseConfig::from_env()?;
    let pool = db_config.create_pool().await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Build application router
    let app = create_app(pool);

    // Start server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(pool: sqlx::PgPool) -> Router {
    Router::new()
        // Public routes
        .route("/api/health", get(handlers::health::health_check))
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        
        // Protected routes
        .route("/api/users/me", get(handlers::users::get_current_user))
        .route("/api/users/me", post(handlers::users::update_current_user))
        
        // Middleware layers
        .layer(axum_middleware::from_fn_with_state(pool.clone(), middleware::auth::auth_middleware))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        
        // Application state
        .with_state(pool)
}