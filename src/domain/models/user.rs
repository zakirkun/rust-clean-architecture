use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Validate)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub is_email_verified: bool,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
}

// Password validation struct
#[derive(Debug, Validate)]
pub struct PasswordRequirements {
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(regex(
        path = "PASSWORD_REGEX",
        message = "Password must contain at least one uppercase letter, one lowercase letter, one number, and one special character"
    ))]
    pub password: String,
}

lazy_static::lazy_static! {
    static ref PASSWORD_REGEX: regex::Regex = regex::Regex::new(
        r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
    ).unwrap();
} 