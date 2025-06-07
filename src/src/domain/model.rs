use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Task {
    pub task_id: i32,
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub name: String,
    pub priority: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub priority: Option<i32>,
}
