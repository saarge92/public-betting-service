use crate::domain::AppError;
use crate::domain::wallet::Wallet;
use crate::infrastructure::util::is_unique_violation;
use crate::operation::wallet::create::dto::CreateWalletDto;
use crate::operation::wallet::errors::WalletError;
use crate::repository::dto::CreateWalletDto as RepositoryCreateWalletDto;
use crate::repository::wallet::WalletRepositoryTrait;
use sea_orm::prelude::async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait CreateWalletServiceTrait: Send + Sync + Interface {
    async fn create(&self, dto: CreateWalletDto, user_id: Uuid) -> Result<Wallet, AppError>;
}

#[derive(Component)]
#[shaku(interface = CreateWalletServiceTrait)]
pub struct CreateWalletService {
    #[shaku(inject)]
    wallet_repo: Arc<dyn WalletRepositoryTrait>,
}

#[async_trait]
impl CreateWalletServiceTrait for CreateWalletService {
    async fn create(&self, dto: CreateWalletDto, user_id: Uuid) -> Result<Wallet, AppError> {
        let wallet_create_dto = RepositoryCreateWalletDto {
            user_id,
            currency: dto.currency,
        };

        self.wallet_repo
            .create(wallet_create_dto)
            .await
            .map_err(|db_err| {
                if is_unique_violation(&db_err) {
                    AppError::from(WalletError::UserWalletAlreadyExists)
                } else {
                    AppError::Internal(db_err.to_string())
                }
            })
    }
}
