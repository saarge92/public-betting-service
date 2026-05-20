use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use crate::domain::{AppError, User};
use crate::operation::user::errors::UserError;
use crate::operation::user::RegisterUserDto;
use crate::repository::UserRepositoryTrait;

pub struct UserService {
    user_repo: Arc<dyn UserRepositoryTrait>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepositoryTrait>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, user_dto: RegisterUserDto) -> Result<User, AppError> {
        if let Some(_) = self.user_repo.find_user_by_username_or_email(user_dto.username.clone(), user_dto.email.clone()).await? {
            return Err(UserError::AlreadyExists.into());
        }

        let password_hash = hash(user_dto.password_raw.clone(), DEFAULT_COST)
            .map_err(|e| AppError::Crypto(e.to_string()))?;

        let new_user = self.user_repo.create_user(user_dto, password_hash.clone()).await?;

        Ok(new_user)
    }
}