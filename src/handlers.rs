use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response}, Json,
};
use sqlx::{Pool, Postgres};

use crate::models;

pub async fn get_todos(State(pool): State<Pool<Postgres>>) -> Response {
    let res = sqlx::query_as::<_, models::Todo>("SELECT * FROM todos;")
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(res)).into_response()
}
