use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config;
mod handlers;
pub mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config::Config { addr, db_conn } = config::Config::new();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn)
        .await
        .expect("Invalid database connection");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(
            "/todos",
            get(handlers::get_todos)
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
