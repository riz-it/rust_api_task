use axum::{Router, routing::{get, patch}};
use std::sync::Arc;
use crate::{
    controller::task_controller::{create_task, delete_task, get_tasks, update_task},
    service::task_service::TaskService,
    domain::task::TaskRepository,
};

pub fn create_routes<R: TaskRepository + 'static>(service: Arc<TaskService<R>>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/{task_id}", patch(update_task).delete(delete_task))
        .with_state(service)
}
