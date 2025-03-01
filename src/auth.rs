use axum:: {
  extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}
};
use auth_lib::validate_token;

pub async fn auth(
  headers: HeaderMap,
  request: Request,
  next: Next,
) -> Response {
  let token = headers.get("auth_token");
  
  if let Some(val) = token {

    let token_str = match val.to_str() {
      Ok(value) => value,
      Err(e) => {
        return (StatusCode::UNAUTHORIZED, format!("auth_token is invalid: {e}")).into_response();
      }
    };
    let is_verified = validate_token(token_str, "1234");
    
    match is_verified {
      Ok(_) => (),
      Err(e) => {
        return (StatusCode::UNAUTHORIZED, format!("auth_token is invalid: {e}")).into_response();
      }
    }

    let response = next.run(request).await;
    response
  } else {
    (StatusCode::UNAUTHORIZED, "auth_token does not exist").into_response()
  }
}