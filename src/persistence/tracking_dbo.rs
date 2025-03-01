use sqlx::PgPool;
use crate::models::{Tracking, TrackingDetail, DBError};
use async_trait::async_trait;

#[async_trait]
pub trait TrackingDbo {
    async fn create_tracking(&self, tracking: Tracking) -> Result<TrackingDetail, DBError>;
    async fn get_tracking(&self, task_uuid: String) -> Result<Vec<TrackingDetail>, DBError>;
    async fn delete_tracking(&self, task_uuid: String) -> Result<(), DBError>;
}

#[derive(Debug)]
pub struct TrackingDboImpl {
    db: PgPool,
}

impl TrackingDboImpl {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
        }
    }
}

#[async_trait]
impl TrackingDbo for TrackingDboImpl {
    async fn create_tracking(&self, tracking: Tracking) -> Result<TrackingDetail, DBError> {
        let task_uuid = sqlx::types::Uuid::parse_str(&tracking.task_uuid).map_err(|e| {
          DBError::InvalidInput(e.to_string())
        })?;
        
        let record = sqlx::query!(
            r#"
            INSERT INTO tracking (task_task_uuid, status)
            VALUES ($1, $2)
            RETURNING *
            "#,
            task_uuid,
            tracking.status,
        ).fetch_one(&self.db).await.map_err(|e| {
            DBError::Other(e.to_string())
        })?;

        Ok(TrackingDetail {
            id: record.id.to_string(),
            task_uuid: tracking.task_uuid,
            status: record.status.to_string(),
            created_at: record.created_at.to_string(),
        })
    }

    async fn get_tracking(&self, task_uuid: String) -> Result<Vec<TrackingDetail>, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&task_uuid).map_err(|e| {
          DBError::InvalidInput(e.to_string())
        })?;

        let records = sqlx::query!(
            r#"
            SELECT * FROM tracking WHERE task_task_uuid = $1
            "#,
            uuid
        ).fetch_all(&self.db).await.map_err(|e| {
            DBError::Other(e.to_string())
        })?;

        Ok(
            records.iter().map(|r| {
                TrackingDetail {
                    id: r.id.to_string(),
                    task_uuid: task_uuid.clone(),
                    status: r.status.to_string(),
                    created_at: r.created_at.to_string(),
                }
            }).collect()
        )
    }

    async fn delete_tracking(&self, task_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&task_uuid).map_err(|e| {
          DBError::InvalidInput(e.to_string())
        })?;

        sqlx::query!(
            r#"
            DELETE FROM tracking WHERE task_task_uuid = $1
            "#,
            uuid
        ).execute(&self.db).await.map_err(|e| {
            DBError::Other(e.to_string())
        })?;

        Ok(())
    }
}
