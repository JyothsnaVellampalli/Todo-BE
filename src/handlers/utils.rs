use auth_lib::validate_token;
use crate::models::DBError;
use axum::http::HeaderMap;

pub fn get_user_from_token(token: &str) -> Result<String, String> {
    let claims = validate_token(token, "1234")?;
    Ok(claims.sub)
}

pub fn validate_user(headers: &HeaderMap) -> Result<String, DBError> {
  let token = headers.get("auth_token")
      .ok_or_else(|| DBError::UnAuthorized("Error reading header auth_token".to_string()))?;

  let user_name = get_user_from_token(
      token.to_str().map_err(|e| DBError::UnAuthorized(format!("Invalid token: {e}")))?
  ).map_err(|e| DBError::UnAuthorized(e))?;

  Ok(user_name)
}

// pub fn validate_user_token(headers: &HeaderMap, user_name: &str) -> Result<(), DBError> {
//   if let Some(token) = headers.get("auth_token") {
//     let user = get_user_from_token(token.to_str().unwrap());
//     if user_name != user {
//       return Err(DBError::Other("Invalid token for user".to_string()));
//     }
//     return Ok(());
//   } else {
//     return Err(DBError::Other("No valid token provided".to_string()));
//   }
// }