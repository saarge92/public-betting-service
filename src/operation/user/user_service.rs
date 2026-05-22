use crate::domain::{AppError, User};
use crate::operation::user::RegisterUserDto;
use crate::operation::user::errors::UserError;
use crate::repository::UserRepositoryTrait;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use sea_orm::prelude::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait UserServiceTrait: Interface + Send + Sync {
    async fn register(&self, user_dto: RegisterUserDto) -> Result<User, AppError>;
}

#[derive(Component)]
#[shaku(interface = UserServiceTrait)]
pub struct UserService {
    #[shaku(inject)]
    user_repo: Arc<dyn UserRepositoryTrait>,
}

#[async_trait::async_trait]
impl UserServiceTrait for UserService {
    async fn register(&self, user_dto: RegisterUserDto) -> Result<User, AppError> {
        if self
            .user_repo
            .find_user_by_username_or_email(user_dto.username.clone(), user_dto.email.clone())
            .await?
            .is_some()
        {
            return Err(UserError::AlreadyExists.into());
        }

        // 1. Генерируем соль автоматически через OsRng (она гарантированно будет правильной длины)
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(user_dto.password_raw.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Ошибка хэширования: {}", e)))?
            .to_string();

        let new_user = self
            .user_repo
            .create_user(user_dto, password_hash.clone())
            .await?;

        Ok(new_user)
    }
}
