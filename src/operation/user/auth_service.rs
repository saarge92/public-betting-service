use crate::api::middlewares::auth::Claims;
use crate::config::AppConfig;
use crate::domain::AppError;
use crate::operation::user::UserError;
use crate::operation::user::dto::{AuthResponseDto, LoginDto};
use crate::repository::UserRepositoryTrait;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, encode};
use sea_orm::prelude::async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait AuthUserServiceTrait: Interface + Send + Sync {
    async fn login(&self, login_dto: LoginDto) -> Result<AuthResponseDto, AppError>;
    async fn authorize_token(&self, token: &str) -> Result<Claims, actix_web::Error>;
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

    async fn authorize_token(&self, token: &str) -> Result<Claims, actix_web::Error> {
        use actix_web::error::ErrorUnauthorized;

        let secret = self.config.jwt_secret.as_bytes();
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::default(),
        )
        .map_err(|_| ErrorUnauthorized("Невалидный токен"))?;

        let claims = token_data.claims;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Internal("Неверный формат UUID".to_string()))?;

        let user_option = self
            .user_repo
            .find_by_id(user_id)
            .await
            .map_err(|e| ErrorUnauthorized(format!("Ошибка базы данных: {}", e)))?;

        match user_option {
            Some(_) => Ok(claims),
            None => Err(ErrorUnauthorized("Пользователь не существует")),
        }
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
