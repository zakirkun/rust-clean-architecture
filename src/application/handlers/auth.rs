use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bcrypt::{verify, hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use crate::{
    infrastructure::{
        auth::jwt::JwtService,
        error::AppError,
    },
    domain::repositories::user_repository::UserRepository,
};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user_id: i32,
    email: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    user_id: i32,
    email: String,
}

pub async fn login<T: UserRepository>(
    State(repo): State<T>,
    State(jwt_service): State<JwtService>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Find user by email
    let user = repo.find_by_email(&payload.email).await?
        .ok_or(AppError::AuthenticationError)?;

    // Verify password
    if !verify(payload.password.as_bytes(), &user.password)
        .map_err(|_| AppError::AuthenticationError)? {
        return Err(AppError::AuthenticationError);
    }

    // Generate JWT token
    let token = jwt_service.generate_token(user.id)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
        email: user.email,
    }))
}

pub async fn register<T: UserRepository>(
    State(repo): State<T>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    // Check if user already exists
    if let Some(_) = repo.find_by_email(&payload.email).await? {
        return Err(AppError::AuthenticationError);
    }

    // Create new user
    let user = repo.create(payload.email, payload.password).await?;

    Ok(Json(RegisterResponse {
        user_id: user.id,
        email: user.email,
    }))
} 