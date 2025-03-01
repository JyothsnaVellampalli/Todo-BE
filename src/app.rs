use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::{get, post, delete, patch},
    Router,
    middleware
};
use axum::routing::IntoMakeService;
use std::sync::Arc;
use crate::persistence::{
    tasks_dbo::{TasksDbo, TasksDboImpl},
    users_dbo::{UsersDbo, UsersDboImpl},
    tracking_dbo::{TrackingDbo, TrackingDboImpl}
};
use crate::handlers::*;
use crate::auth::auth;
use crate::logging::logging_middleware;

#[derive(Clone)]
pub struct AppState {
    pub tasks_dbo: Arc<dyn TasksDbo + Send + Sync>,
    pub users_dbo: Arc<dyn UsersDbo + Send + Sync>,
    pub tracking_dbo: Arc<dyn TrackingDbo + Send + Sync>,
}

pub async fn prepare_app() -> IntoMakeService<Router>{
  let pool = PgPoolOptions::new()
      .max_connections(5) // Set max connection pool size
      .connect(&std::env::var("DATABASE_URL").expect("Invalid database URL"))
      .await.expect("Unable to create postgres connection pool");

  let tasks_dbo = Arc::new(TasksDboImpl::new(pool.clone()));
  let users_dbo = Arc::new(UsersDboImpl::new(pool.clone()));
  let tracking_dbo = Arc::new(TrackingDboImpl::new(pool));

  let app_state = AppState {
      tasks_dbo,
      users_dbo,
      tracking_dbo
  };

  let app = Router::new()
      .route("/", get(get_all))
      .route("/:id", get(get_task))
      .route("/", post(add_task))
      .route("/", patch(update_task))
      .route("/update-status", patch(update_status))
      .route("/", delete(delete_task))
      .route_layer(middleware::from_fn(auth))
      .route("/register", post(register_user))
      .route("/login", post(login))
      .layer(middleware::from_fn(logging_middleware))
      .with_state(app_state);
  
  app.into_make_service()
}