use serde::Serialize;

use crate::TodoReq;

#[derive(Clone, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub text: String,
    pub done: bool,
}

#[derive(Clone)]
pub struct State {
    pub todos: Vec<Todo>,
}

impl Todo {
    fn from(todo_req: TodoReq, id: u64) -> Self {
        Self {
            id,
            title: todo_req.title,
            text: todo_req.text,
            done: todo_req.done,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn add(&mut self, todo: Todo) -> Todo {
        self.todos.push(todo);
        self.todos.last().unwrap().clone()
    }

    pub fn update(&mut self, todo: TodoReq, index: usize) -> Todo {
        self.todos[index] = Todo::from(todo, self.todos.len() as u64);
        self.todos[index].clone()
    }

    pub fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }
}
