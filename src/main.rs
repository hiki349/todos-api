use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;

mod config;
mod handlers;
pub mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config::Config { addr } = config::Config::new();

    let state = state::State::new();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/todos", get(handlers::index))
        .route("/todos/:id", get(handlers::show))
        .route("/todos", post(handlers::create))
        .route("/todos/:id", put(handlers::update))
        .route("/todos/:id", delete(handlers::destroy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap()
}
