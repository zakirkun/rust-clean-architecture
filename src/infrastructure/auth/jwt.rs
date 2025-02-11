use crate::domain::models::user::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::env;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Failed to create token")]
    TokenCreation,
    #[error("Failed to verify token")]
    TokenVerification,
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn generate_token(&self, user_id: i32) -> Result<String, JwtError> {
        let now = chrono::Utc::now().timestamp() as usize;
        let claims = Claims {
            sub: user_id,
            exp: now + 24 * 3600, // 24 hours from now
            iat: now,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| JwtError::TokenCreation)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, JwtError> {
        let validation = Validation::default();
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| JwtError::TokenVerification)
    }
} 