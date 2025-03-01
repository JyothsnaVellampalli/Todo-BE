use axum::{
    extract::State as AxumState,
    extract::Path,
    response::IntoResponse,
    http::StatusCode,
    Json as JsonAxum,
    http::HeaderMap,
};
use crate::models::*;
use crate::app::AppState;
use auth_lib::{generate_token, hash_password, verify_password};

mod utils;

use utils::validate_user;

impl IntoResponse for DBError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DBError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            DBError::UnAuthorized(msg) => {
                (StatusCode::UNAUTHORIZED, msg).into_response()
            }
            DBError::Other(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

pub async fn register_user(
    AxumState(AppState { users_dbo, .. }): AxumState<AppState>,
    JsonAxum(user): JsonAxum<User>
) -> Result<impl IntoResponse, DBError> {
    let user = User {
        username: user.username,
        password: hash_password(&user.password).map_err(|e| DBError::Other(e))?,
        email: user.email
    };
    let user = users_dbo.create_user(user).await?;

    let token: String  = generate_token(&user.username, "1234").map_err(|e| DBError::Other(e))?;

    if token == "" {
        return Err(DBError::Other("Failed to generate token".to_string()));
    }

    Ok(JsonAxum(UserToken { token }))
}

pub async fn login(
    AxumState(AppState { users_dbo, .. }): AxumState<AppState>,
    JsonAxum(user): JsonAxum<LoginReq>
) -> Result<impl IntoResponse, DBError>{
    println!("step1");
    let user_stored = users_dbo.get_user(user.username).await?;
    println!("step2");
    let is_verified = verify_password(&user.password, &user_stored.password);
    println!("step3");
    if is_verified {
        println!("step4");
        let token: String = generate_token(&user_stored.username, "1234").map_err(|e| DBError::Other(e))?;
        return Ok(JsonAxum(UserToken { token }));
    } else {
        println!("step5");
        return Err(DBError::InvalidInput("Invalid password".to_string()));
    }
}

// TODO: update to User-based fetching.
pub async fn get_all(
    headers: HeaderMap,
    AxumState(AppState { tasks_dbo , ..}): AxumState<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let user_name = validate_user(&headers)?;
    tasks_dbo.get_all_tasks(user_name).await.map(JsonAxum)
}

pub async fn get_task(
    headers: HeaderMap,
    Path(id): Path<String>,
    AxumState(AppState { tasks_dbo, tracking_dbo, .. }): AxumState<AppState>
) -> Result<impl IntoResponse, DBError> {
    println!("ID: {}", id);
    let user_name = validate_user(&headers)?;
    let task = tasks_dbo.get_task(&id, user_name).await?;
    let tracking = tracking_dbo.get_tracking(id).await?;

    if tracking.len() == 0 {
        return Ok(JsonAxum(TaskDetailResponse {
            task,
            tracking: None
        }));
    } else {
        let resp = TaskDetailResponse {
            task,
            tracking: Some(tracking)
        };

    Ok(JsonAxum(resp))
    }
}

pub async fn add_task(
    headers: HeaderMap,
    AxumState(AppState { tasks_dbo, tracking_dbo , ..}): AxumState<AppState>,
    JsonAxum(task): JsonAxum<Task>
) -> Result<impl IntoResponse, DBError> {
    let user_name = validate_user(&headers)?;
    let task = tasks_dbo.create_task(task, user_name).await?;

    #[allow(unused)]
    tracking_dbo.create_tracking(Tracking {
            task_uuid: task.task_uuid.clone(),
            status: format!("Task Created with status {}", task.status.to_string())
    }).await;

    Ok(JsonAxum(task))
}

pub async fn update_task(
    headers: HeaderMap,
    AxumState(AppState { tasks_dbo, tracking_dbo, .. }): AxumState<AppState>,
    JsonAxum(task): JsonAxum<TaskUpdateReq>
) -> Result<impl IntoResponse, DBError>{
    let user_name = validate_user(&headers)?;
    let task = tasks_dbo.update_task(task, user_name).await?;

    #[allow(unused)]
    tracking_dbo.create_tracking(Tracking {
        task_uuid: task.task_uuid.clone(),
        status: format!("Task Updated with title: {}, description: {}", task.title.to_string(), task.description.to_string())
    }).await;

    Ok(JsonAxum(task))
}

pub async fn update_status(
    headers: HeaderMap,
    AxumState(AppState { tasks_dbo, tracking_dbo, .. }): AxumState<AppState>,
    JsonAxum(task): JsonAxum<TaskStatusReq>
) -> Result<impl IntoResponse, DBError> {
    let user_name = validate_user(&headers)?;
    let task = tasks_dbo.update_task_status(task.status, task.task_uuid, user_name).await?;

    #[allow(unused)]
    tracking_dbo.create_tracking(Tracking {
            task_uuid: task.task_uuid.clone(),
            status: format!("Task Updated with status {}", task.status.to_string())
    }).await;

    Ok(JsonAxum(task))
}

pub async fn delete_task(
    headers: HeaderMap,
    AxumState(AppState { tasks_dbo, tracking_dbo , ..}): AxumState<AppState>,
    JsonAxum(task): JsonAxum<TaskId>
) -> Result<impl IntoResponse, DBError> {
    let user_name = validate_user(&headers)?;
    #[allow(unused)]
    tasks_dbo.delete_task(task.task_uuid.clone(), user_name).await;
    #[allow(unused)]
    tracking_dbo.delete_tracking(task.task_uuid).await;

    Ok(())
}
