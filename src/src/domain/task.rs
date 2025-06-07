use async_trait::async_trait;
use crate::domain::model::{Task, CreateTask, UpdateTask};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Task>, String>;
    async fn create(&self, task: CreateTask) -> Result<i32, String>;
    async fn update(&self, id: i32, task: UpdateTask) -> Result<(), String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
}
