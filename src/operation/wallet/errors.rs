use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Кошелек с такой валютой уже существует для данного пользователя")]
    UserWalletAlreadyExists,
    #[error("Кошелек не найден")]
    NotFound,
}
