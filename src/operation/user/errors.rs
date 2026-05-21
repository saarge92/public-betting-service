// src/services/user/errors.rs
#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("Пользователь уже существует")]
    AlreadyExists,

    #[error("Пользователь с ID {0} не найден")]
    NotFound(String),

    #[error("Неверный пароль")]
    WrongPassword,

    #[error("Неверный логин или пароль")]
    Unauthorized
}