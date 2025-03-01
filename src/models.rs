use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

impl TaskStatus {
    pub fn from_str(s: &str) -> Result<TaskStatus, DBError> {
        match s {
            "todo" => Ok(TaskStatus::Todo),
            "in_progress" => Ok(TaskStatus::InProgress),
            "done" => Ok(TaskStatus::Done),
            _ => Err(DBError::Other(format!("Invalid status: {}", s))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "todo".to_string(),
            TaskStatus::InProgress => "in_progress".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub status: TaskStatus
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskUpdateReq {
    pub task_uuid: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskStatusReq {
    pub task_uuid: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskId {
    pub task_uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskDetail {
    pub task_uuid: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub user_name: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetail {
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tracking {
   pub status: String,
   pub task_uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct TrackingDetail {
    pub id: String,
    pub status: String,
    pub task_uuid: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDetailResponse {
    pub task: TaskDetail,
    pub tracking: Option<Vec<TrackingDetail>>,
}

#[derive(Debug)]
pub enum DBError {
  InvalidInput(String),
  UnAuthorized(String),
  Other(String),
}