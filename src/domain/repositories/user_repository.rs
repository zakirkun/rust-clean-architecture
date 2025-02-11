use async_trait::async_trait;
use crate::domain::models::user::{User, UserRole};
use crate::infrastructure::error::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, email: String, password: String, role: UserRole) -> Result<User, AppError>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn update(
        &self, 
        id: i32, 
        email: Option<String>, 
        password: Option<String>,
        role: Option<UserRole>,
    ) -> Result<User, AppError>;
    async fn soft_delete(&self, id: i32) -> Result<bool, AppError>;
    async fn list(&self, limit: i64, offset: i64, include_deleted: bool) -> Result<Vec<User>, AppError>;
    async fn verify_email(&self, id: i32) -> Result<User, AppError>;
} 