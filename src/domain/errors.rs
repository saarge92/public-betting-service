// src/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // Автоматически оборачивает любые ошибки из модуля User
    #[error("Ошибка пользователя: {0}")]
    User(#[from] crate::operation::user::UserError),

    // Системные ошибки, общие для всего приложения
    #[error("Ошибка базы данных: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Ошибка хэширования: {0}")]
    Crypto(String),
}