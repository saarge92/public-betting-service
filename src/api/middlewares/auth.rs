use crate::container::AppContainer;
use crate::operation::user::auth_service::AuthUserServiceTrait;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{FromRequest, HttpRequest, web};
use futures_util::FutureExt;
use futures_util::future::BoxFuture;
use serde::{Deserialize, Serialize};
use shaku::HasComponent;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let container = match req.app_data::<web::Data<AppContainer>>() {
            Some(c) => c.clone(),
            None => {
                return async {
                    Err(ErrorUnauthorized("Контейнер зависимостей не найден"))
                }
                .boxed();
            }
        };

        let auth_service: Arc<dyn AuthUserServiceTrait> = container.resolve();

        // Вытаскиваем заголовок
        let auth_header = match req
            .headers()
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
        {
            Some(s) => s.to_string(),
            None => return async { Err(ErrorUnauthorized("Токен отсутствует")) }.boxed(),
        };

        if !auth_header.starts_with("Bearer ") {
            return async {
                Err(ErrorUnauthorized("Тип токена должен быть Bearer"))
            }
            .boxed();
        }

        let token = auth_header[7..].to_string();

        // Просто делегируем задачу сервису
        async move { auth_service.authorize_token(&token).await }.boxed()
    }
}
