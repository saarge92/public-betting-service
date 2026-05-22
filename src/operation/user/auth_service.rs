use crate::config::AppConfig;
use crate::domain::AppError;
use crate::operation::user::UserError;
use crate::operation::user::dto::{AuthResponseDto, Claims, LoginDto};
use crate::repository::UserRepositoryTrait;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::prelude::async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait AuthUserServiceTrait: Interface + Send + Sync {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError>;
}

#[derive(Component)]
#[shaku(interface = AuthUserServiceTrait)]
pub struct AuthService {
    config: AppConfig,

    #[shaku(inject)]
    user_repo: Arc<dyn UserRepositoryTrait>,
}

#[async_trait]
impl AuthUserServiceTrait for AuthService {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError> {
        let user = self
            .user_repo
            .find_user_by_username_or_email(login_dto.user.clone(), login_dto.user.clone())
            .await?;

        let user = user.ok_or(UserError::Unauthorized)?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if Argon2::default()
            .verify_password(login_dto.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(UserError::Unauthorized.into());
        }

        Ok(AuthResponseDto {
            token: self.sign_token(&user)?,
            token_type: "Bearer".to_string(),
        })
    }
}

impl AuthService {
    fn sign_token(&self, user: &crate::domain::User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Ошибка генерации JWT: {}", e)))?;

        Ok(token)
    }
}
