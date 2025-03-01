use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand_core::OsRng;
use argon2::{Argon2, password_hash::{PasswordHasher, SaltString, PasswordVerifier, PasswordHash}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (Username)
    exp: usize,   // Expiration timestamp
}

/// Generates a JWT token based on username
pub fn generate_token(username: &str, secret: &str) -> Result<String, String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() + 3600; // 1-hour expiry

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|e| e.to_string())
}

/// Validates a JWT token
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).map_err(|e| e.to_string())?;
    Ok(token_data.claims)
}

/// Hash a password securely
pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng); // Generate a random salt

    let hash =Argon2::default().hash_password(password.as_bytes(), &salt);
    match hash {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(e.to_string())
    }
}

/// Verify a password against a stored hash
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(stored_hash);
    match parsed_hash {
        Err(_) => return false,
        Ok(hash) => Argon2::default().verify_password(password.as_bytes(), &hash).is_ok()
    }
}

