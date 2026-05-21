use crate::domain::AppError;
use crate::operation::user::UserError;
use crate::operation::user::dto::{AuthResponseDto, Claims, LoginDto};
use crate::repository::UserRepositoryTrait;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::prelude::async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

#[async_trait]
pub trait AuthUserServiceTrait: Interface + Send + Sync {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError>;
}

#[derive(Component)]
#[shaku(interface = AuthUserServiceTrait)]
pub struct AuthService {
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

        let user = user.ok_or_else(|| UserError::Unauthorized.into())?;

        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if Argon2::default()
            .verify_password(login_dto.password.as_bytes(), &parsed_hash)
            .is_err()
        {
            return Err(UserError::Unauthorized.into());
        }

        // 3. Генерируем JWT Token
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24)) // Токен на 24 часа
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: user.id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        ).map_err(|e| {
            log::error!("Ошибка генерации JWT: {:?}", e);
            AppError::Internal
        })?;

        Ok(AuthResponseDto {
            token,
            token_type: "Bearer".to_string(),
        })
    }
}
