use async_trait::async_trait;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use crate::{
    domain::{
        models::user::User,
        repositories::user_repository::UserRepository,
    },
    infrastructure::{
        error::AppError,
        config::database::DbPool,
    },
};
use validator::Validate;

pub struct DieselUserRepository {
    pool: DbPool,
}

impl DieselUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn validate_password(&self, password: &str) -> Result<(), AppError> {
        let requirements = PasswordRequirements {
            password: password.to_string(),
        };

        requirements.validate().map_err(|_| AppError::InvalidPassword)?;
        Ok(())
    }
}

#[async_trait]
impl UserRepository for DieselUserRepository {
    async fn create(&self, email: String, password: String, role: UserRole) -> Result<User, AppError> {
        use crate::schema::users;
        
        // Validate email
        let user = User {
            id: 0,
            email: email.clone(),
            password: String::new(),
            role,
            is_email_verified: false,
            deleted_at: None,
            created_at: chrono::Utc::now().naive_utc(),
        };
        user.validate().map_err(|_| AppError::InvalidEmail)?;

        // Validate password
        self.validate_password(&password)?;

        let hashed_password = hash(password.as_bytes(), DEFAULT_COST)
            .map_err(|_| AppError::InternalServerError)?;

        let new_user = User {
            password: hashed_password,
            ..user
        };

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        users.find(id)
            .first(conn)
            .optional()
            .map_err(AppError::DatabaseError)
    }

    async fn find_by_email(&self, email_query: &str) -> Result<Option<User>, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        users.filter(email.eq(email_query))
            .first(conn)
            .optional()
            .map_err(AppError::DatabaseError)
    }

    async fn update(&self, user_id: i32, email_update: Option<String>, password_update: Option<String>) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        let mut update = diesel::update(users.find(user_id));

        if let Some(email_val) = email_update {
            update = update.set(email.eq(email_val));
        }

        if let Some(password_val) = password_update {
            let hashed_password = hash(password_val.as_bytes(), DEFAULT_COST)
                .map_err(|_| AppError::InternalServerError)?;
            update = update.set(password.eq(hashed_password));
        }

        update
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }

    async fn delete(&self, user_id: i32) -> Result<bool, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        let deleted = diesel::delete(users.find(user_id))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;

        Ok(deleted > 0)
    }

    async fn soft_delete(&self, user_id: i32) -> Result<bool, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        let now = chrono::Utc::now().naive_utc();
        let updated = diesel::update(users.find(user_id))
            .set(deleted_at.eq(now))
            .execute(conn)
            .map_err(AppError::DatabaseError)?;

        Ok(updated > 0)
    }

    async fn list(&self, limit_val: i64, offset_val: i64, include_deleted: bool) -> Result<Vec<User>, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        let mut query = users.into_boxed();
        
        if !include_deleted {
            query = query.filter(deleted_at.is_null());
        }

        query
            .limit(limit_val)
            .offset(offset_val)
            .load(conn)
            .map_err(AppError::DatabaseError)
    }

    async fn verify_email(&self, user_id: i32) -> Result<User, AppError> {
        use crate::schema::users::dsl::*;

        let conn = &mut self.pool.get()
            .map_err(|_| AppError::DatabaseError(diesel::result::Error::NotFound))?;

        diesel::update(users.find(user_id))
            .set(is_email_verified.eq(true))
            .get_result(conn)
            .map_err(AppError::DatabaseError)
    }
} 