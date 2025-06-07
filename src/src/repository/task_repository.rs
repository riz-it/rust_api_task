use sqlx::PgPool;
use async_trait::async_trait;
use crate::domain::{model::{Task, CreateTask, UpdateTask}, task::TaskRepository};

pub struct TaskRepositoryImpl {
    pub db: PgPool,
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn get_all(&self) -> Result<Vec<Task>, String> {
        sqlx::query_as!(Task, "SELECT * FROM tasks ORDER BY task_id")
            .fetch_all(&self.db)
            .await
            .map_err(|e| e.to_string())
    }

    async fn create(&self, task: CreateTask) -> Result<i32, String> {
        let row = sqlx::query!(
            "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
            task.name,
            task.priority
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.task_id)
    }

    async fn update(&self, id: i32, task: UpdateTask) -> Result<(), String> {
        let mut query = "UPDATE tasks SET task_id = $1".to_string();
        let mut i = 2;

        if task.name.is_some() {
            query.push_str(&format!(", name = ${i}"));
            i += 1;
        }
        if task.priority.is_some() {
            query.push_str(&format!(", priority = ${i}"));
        }

        query.push_str(" WHERE task_id = $1");

        let mut s = sqlx::query(&query).bind(id);
        if let Some(name) = task.name {
            s = s.bind(name);
        }
        if let Some(priority) = task.priority {
            s = s.bind(priority);
        }

        s.execute(&self.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        sqlx::query!("DELETE FROM tasks WHERE task_id = $1", id)
            .execute(&self.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
