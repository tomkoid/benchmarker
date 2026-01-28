mod models;
mod routes;

use axum::{
    Router,
    http::{
        HeaderValue,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::Any;

use crate::db::ensure_database_exists;

pub mod compare;
pub mod db;
pub mod schemas;

#[tokio::main]
pub async fn main() -> color_eyre::Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Ensure database exists before connecting
    ensure_database_exists(&database_url).await?;

    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("Syncing category schemas from schemas.yml...");
    schemas::sync_categories(&pool).await?;

    let state = routes::AppState { db: pool };

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .nest("/api/v1", routes::api_routes())
        .layer(cors_layer)
        .with_state(state);

    let bind_addr = format!("{}:{}", host, port);
    tracing::info!("Server listening on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
