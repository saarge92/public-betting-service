use crate::domain::AppError;
use crate::domain::wallet::Wallet;
use crate::repository::wallet::WalletRepositoryTrait;
use sea_orm::prelude::async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait WalletListServiceTrait: Interface + Sync + Send {
    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Wallet>, AppError>;
}

#[derive(Component)]
#[shaku(interface=WalletListServiceTrait)]
pub struct WalletListService {
    #[shaku(inject)]
    wallet_repo: Arc<dyn WalletRepositoryTrait>,
}

#[async_trait]
impl WalletListServiceTrait for WalletListService {
    async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Wallet>, AppError> {
        Ok(self.wallet_repo.user_wallets(user_id).await?)
    }
}
