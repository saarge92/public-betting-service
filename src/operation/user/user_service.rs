use crate::domain::{AppError, User};
use crate::operation::user::RegisterUserDto;
use crate::operation::user::errors::UserError;
use crate::repository::UserRepositoryTrait;
use bcrypt::{DEFAULT_COST, hash};
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
        if let Some(_) = self
            .user_repo
            .find_user_by_username_or_email(user_dto.username.clone(), user_dto.email.clone())
            .await?
        {
            return Err(UserError::AlreadyExists.into());
        }

        let password_hash = hash(user_dto.password_raw.clone(), DEFAULT_COST)
            .map_err(|e| AppError::Crypto(e.to_string()))?;

        let new_user = self
            .user_repo
            .create_user(user_dto, password_hash.clone())
            .await?;

        Ok(new_user)
    }
}
