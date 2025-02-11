use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::{
    domain::repositories::user_repository::UserRepository,
    infrastructure::error::AppError,
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<crate::domain::models::user::User> for UserResponse {
    fn from(user: crate::domain::models::user::User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            created_at: user.created_at,
        }
    }
}

pub async fn create_user<T: UserRepository>(
    State(repo): State<T>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = repo.create(payload.email, payload.password).await?;
    Ok(Json(user.into()))
}

pub async fn get_user<T: UserRepository>(
    State(repo): State<T>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = repo.find_by_id(id).await?
        .ok_or(AppError::NotFound)?;
    Ok(Json(user.into()))
}

pub async fn update_user<T: UserRepository>(
    State(repo): State<T>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = repo.update(id, payload.email, payload.password).await?;
    Ok(Json(user.into()))
}

pub async fn delete_user<T: UserRepository>(
    State(repo): State<T>,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    repo.delete(id).await?;
    Ok(())
}

pub async fn list_users<T: UserRepository>(
    State(repo): State<T>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);
    
    let users = repo.list(limit, offset).await?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
} 