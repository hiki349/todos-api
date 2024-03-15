use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config;
mod handlers;
pub mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config::Config { addr, db_conn } = config::Config::new();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn).await.unwrap();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/todos", get(handlers::index))
        .route("/todos/:id", get(handlers::show))
        .route("/todos", post(handlers::create))
        .route("/todos/:id", put(handlers::update))
        .route("/todos/:id", delete(handlers::destroy));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap()
}
