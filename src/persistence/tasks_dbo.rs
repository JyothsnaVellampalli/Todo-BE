use sqlx::PgPool;
use crate::models::{Task, TaskStatus, TaskDetail, TaskUpdateReq, DBError};
use async_trait::async_trait;
use async_graphql::*;


#[async_trait]
pub trait TasksDbo {
  async fn get_all_tasks(&self, user: String) -> Result<Vec<TaskDetail>, DBError>;
  async fn create_task(&self, task: Task, user: String) -> Result<TaskDetail, DBError>;
  async fn get_task(&self, task_uuid: &str, user: String) -> Result<TaskDetail, DBError>;
  async fn update_task(&self, task: TaskUpdateReq, user: String) -> Result<TaskDetail, DBError>;
  async fn update_task_status(&self, task_status: TaskStatus, task_uuid: String, user: String) -> Result<TaskDetail, DBError>;
  async fn delete_task(&self, task_uuid: String, user: String) -> Result<(), DBError>;
}

#[derive(Debug, Clone)]
pub struct TasksDboImpl {
  db: PgPool,
}

impl TasksDboImpl {
  pub fn new(db: PgPool) -> Self {
    Self {
      db,
    }
  }
}

#[async_trait]
impl TasksDbo for TasksDboImpl {
  async fn get_all_tasks(&self, user: String) -> Result<Vec<TaskDetail>, DBError> {
    let records = sqlx::query!(
        r#"
        SELECT * FROM tasks WHERE user_username = $1
        "#,
        user
    ).fetch_all(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    println!("Records: {:?}", records);

    Ok(
      records.iter().map(|r| {
        TaskDetail {
          task_uuid: r.task_uuid.to_string(),
          title: r.title.to_string(),
          description: r.description.to_string(),
          status: TaskStatus::from_str(&r.status).unwrap(),
          user_name: user.clone(),
          created_at: r.created_at.to_string(),
        }
      }).collect()
    )
  }

  async fn create_task(&self, task: Task, user: String) -> Result<TaskDetail, DBError> {
    let record = sqlx::query!(
        r#"
        INSERT INTO tasks (title, description, status, user_username)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        task.title,
        task.description,
        task.status.to_string(),
        user
    ).fetch_one(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    Ok(
      TaskDetail {
        task_uuid: record.task_uuid.to_string(),
        title: record.title,
        description: record.description,
        status: TaskStatus::from_str(&record.status)?,
        user_name: user,
        created_at: record.created_at.to_string(),
      }
    )
  }

  async fn get_task(&self, task_uuid: &str, user: String) -> Result<TaskDetail, DBError> {
    let uuid = sqlx::types::Uuid::parse_str(task_uuid).map_err(|e| {
      DBError::InvalidInput(e.to_string())
    })?;

    let records = sqlx::query!(
        r#"
        SELECT * FROM tasks WHERE task_uuid = $1 AND user_username = $2
        "#,
        uuid,
        user
    ).fetch_all(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    let r = &records[0];

    Ok(
      TaskDetail {
        task_uuid: r.task_uuid.to_string(),
        title: r.title.to_string(),
        description: r.description.to_string(),
        status: TaskStatus::from_str(&r.status).unwrap(),
        user_name: "".to_string(),
        created_at: r.created_at.to_string(),
      }
    )
  } 

  async fn update_task(&self, task: TaskUpdateReq, user: String) -> Result<TaskDetail, DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&task.task_uuid).map_err(|e| {
      DBError::InvalidInput(e.to_string())
    })?;

    let record = sqlx::query!(
        r#"
        UPDATE tasks
        SET title = $1, description = $2, status = $3
        WHERE task_uuid = $4 AND user_username = $5
        RETURNING *
        "#,
        task.title,
        task.description,
        task.status.to_string(),
        uuid,
        user
    ).fetch_one(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    Ok(
      TaskDetail {
        task_uuid: record.task_uuid.to_string(),
        title: record.title,
        description: record.description,
        status: TaskStatus::from_str(&record.status)?,
        user_name: user,
        created_at: record.created_at.to_string(),
      }
    )
  }

  async fn update_task_status(&self, task_status: TaskStatus, task_uuid: String, user: String) -> Result<TaskDetail, DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&task_uuid).map_err(|e| {
      DBError::InvalidInput(e.to_string())
    })?;

    let record = sqlx::query!(
        r#"
        UPDATE tasks
        SET status = $1
        WHERE task_uuid = $2 AND user_username = $3
        RETURNING *
        "#,
        task_status.to_string(),
        uuid,
        user
    ).fetch_one(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;
    Ok(
      TaskDetail {
        task_uuid: record.task_uuid.to_string(),
        title: record.title,
        description: record.description,
        status: TaskStatus::from_str(&record.status)?,
        user_name: user,
        created_at: record.created_at.to_string(),
      }
    )
  }

  async fn delete_task(&self, task_uuid: String, user: String) -> Result<(), DBError> {
    let uuid = sqlx::types::Uuid::parse_str(&task_uuid).map_err(|e| {
      DBError::InvalidInput(e.to_string())
    })?;

    sqlx::query!(
        r#"
        DELETE FROM tasks WHERE task_uuid = $1 AND user_username = $2
        "#,
        uuid,
        user
    ).execute(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    Ok(())
  }
}