use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Ошибка пользователя: {0}")]
    User(#[from] crate::operation::user::UserError),

    #[error("Ошибка базы данных: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Ошибка хэширования: {0}")]
    Crypto(String),

    #[error("Внутренняя ошибка сервера: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::User(user_err) => match user_err {
                crate::operation::user::UserError::AlreadyExists => StatusCode::CONFLICT,
                _ => StatusCode::BAD_REQUEST,
            },

            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Crypto(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // 2. Формирование тела ответа для клиента
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        match self {
            AppError::Database(db_err) => {
                log::error!("Database infrastructure error: {:?}", db_err);
                HttpResponse::build(status).json(serde_json::json!({
                    "error": "Внутренняя ошибка сервера"
                }))
            }
            AppError::Crypto(msg) => {
                log::error!("Crypto error: {}", msg);
                HttpResponse::build(status).json(serde_json::json!({
                    "error": "Внутренняя ошибка сервера"
                }))
            }

            AppError::User(_) => HttpResponse::build(status).json(serde_json::json!({
                "error": self.to_string()
            })),
            AppError::Internal(_) => HttpResponse::build(status).json(serde_json::json!({
                "error": self.to_string()
            })),
        }
    }
}
