use sqlx::PgPool;
use async_trait::async_trait;
use crate::models::{User, UserDetail, DBError};

#[async_trait]
pub trait UsersDbo {
  async fn create_user(&self, user: User) -> Result<User, DBError>;
  async fn get_user(&self, username: String) -> Result<UserDetail, DBError>;
}

pub struct UsersDboImpl {
  db: PgPool,
}

impl UsersDboImpl {
  pub fn new(db: PgPool) -> Self {
    Self {
      db,
    }
  }
}

#[async_trait]
impl UsersDbo for UsersDboImpl {
  async fn create_user(&self, user: User) -> Result<User, DBError> {
    let record = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        user.username,
        user.email,
        user.password,
    ).fetch_one(&self.db).await.map_err(|e| {
      DBError::Other(e.to_string())
    })?;

    Ok(User {
      username: record.username.to_string(),
      email: record.email.to_string(),
      password: record.password.to_string(),
    })
  }

  async fn get_user(&self, username: String) -> Result<UserDetail, DBError> {
    let record = sqlx::query!(
        r#"
        SELECT * FROM users
        WHERE username = $1
        "#,
        username
    ).fetch_one(&self.db).await.map_err(|e| {
      println!("Error: {}", e);
      DBError::Other(e.to_string())
    })?;

    Ok(UserDetail {
      username: record.username.to_string(),
      email: record.email.to_string(),
      password: record.password.to_string(),
      created_at: record.created_at.to_string(),
    })
  }
}