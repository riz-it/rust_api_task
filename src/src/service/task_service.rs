use crate::domain::{model::{Task, CreateTask, UpdateTask}, task::TaskRepository};
use std::sync::Arc;

pub struct TaskService<R: TaskRepository> {
    pub repo: Arc<R>,
}

impl<R: TaskRepository> TaskService<R> {
    pub async fn get_tasks(&self) -> Result<Vec<Task>, String> {
        self.repo.get_all().await
    }

    pub async fn create_task(&self, task: CreateTask) -> Result<i32, String> {
        self.repo.create(task).await
    }

    pub async fn update_task(&self, id: i32, task: UpdateTask) -> Result<(), String> {
        self.repo.update(id, task).await
    }

    pub async fn delete_task(&self, id: i32) -> Result<(), String> {
        self.repo.delete(id).await
    }
}
