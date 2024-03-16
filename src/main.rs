use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config::Config { addr, db_conn } = config::Config::new();
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn)
        .await
        .unwrap();

    tracing_subscriber::fmt::init();

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
