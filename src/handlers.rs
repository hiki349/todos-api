use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::state;

pub async fn index(
    axum::extract::State(state): axum::extract::State<state::State>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(state.todos)).into_response()
}

pub async fn show(
    axum::extract::State(state): axum::extract::State<state::State>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let todo = state.todos.iter().find(|&todo| todo.id == id);

    match todo {
        Some(todo) => (StatusCode::OK, Json(todo.clone())).into_response(),
        None => {
            let msg = json!({"msg": "not found"});

            (StatusCode::NOT_FOUND, Json(msg)).into_response()
        }
    }
}

pub async fn create(
    axum::extract::State(mut state): axum::extract::State<state::State>,
    Json(payload): Json<state::TodoReq>,
) -> impl IntoResponse {
    let new_todo = state::Todo {
        id: state.todos.len() as u64,
        title: payload.title,
        text: payload.text,
        done: payload.done,
    };
    state.add(new_todo.clone());

    (StatusCode::CREATED, Json(new_todo)).into_response()
}

pub async fn update(
    axum::extract::State(mut state): axum::extract::State<state::State>,
    Path(id): Path<u64>,
    Json(payload): Json<state::TodoReq>,
) -> impl IntoResponse {
    let index = state.todos.iter().position(|todo| todo.id == id);

    match index {
        Some(index) => {
            let todo = state.update(payload, index);

            (StatusCode::OK, Json(todo)).into_response()
        }
        None => {
            let msg = json!({"msg": "not found"});

            (StatusCode::NOT_FOUND, Json(msg)).into_response()
        }
    }
}

pub async fn destroy(
    axum::extract::State(mut state): axum::extract::State<state::State>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let index = state.todos.iter().position(|todo| todo.id == id);

    match index {
        Some(index) => {
            state.remove(index);
            (StatusCode::OK, Json(json!({"msg": "Success delete"}))).into_response()
        }
        None => (StatusCode::NOT_FOUND, Json(json!({"msg": "Not found"}))).into_response(),
    }
}
