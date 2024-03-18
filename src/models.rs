use serde::Serialize;

#[derive(Serialize)]
#[derive(sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
