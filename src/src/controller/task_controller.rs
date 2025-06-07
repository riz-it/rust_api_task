use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{domain::model::{CreateTask, UpdateTask}, service::task_service::TaskService};

pub async fn get_tasks(
    State(service): State<Arc<TaskService<impl crate::domain::task::TaskRepository>>>,
) -> (StatusCode, String) {
    match service.get_tasks().await {
        Ok(data) => (StatusCode::OK, json!({"success": true, "data": data}).to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e}).to_string()),
    }
}

pub async fn create_task(
    State(service): State<Arc<TaskService<impl crate::domain::task::TaskRepository>>>,
    Json(task): Json<CreateTask>,
) -> (StatusCode, String) {
    match service.create_task(task).await {
        Ok(id) => (StatusCode::CREATED, json!({"success": true, "data": {"task_id": id}}).to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e}).to_string()),
    }
}

pub async fn update_task(
    State(service): State<Arc<TaskService<impl crate::domain::task::TaskRepository>>>,
    Path(id): Path<i32>,
    Json(task): Json<UpdateTask>,
) -> (StatusCode, String) {
    match service.update_task(id, task).await {
        Ok(_) => (StatusCode::OK, json!({"success": true}).to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e}).to_string()),
    }
}

pub async fn delete_task(
    State(service): State<Arc<TaskService<impl crate::domain::task::TaskRepository>>>,
    Path(id): Path<i32>,
) -> (StatusCode, String) {
    match service.delete_task(id).await {
        Ok(_) => (StatusCode::OK, json!({"success": true}).to_string()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "message": e}).to_string()),
    }
}
