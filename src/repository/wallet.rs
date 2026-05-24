use crate::domain::wallet::{Wallet, WalletActiveModel};
use crate::repository::dto::CreateWalletDto;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};
use shaku::{Component, Interface};
use uuid::Uuid;

#[async_trait]
pub trait WalletRepositoryTrait: Interface + Send + Sync {
    async fn create(&self, dto: CreateWalletDto) -> Result<Wallet, DbErr>;
}

#[derive(Component)]
#[shaku(interface = WalletRepositoryTrait)]
pub struct WalletRepository {
    #[shaku(default)]
    db_conn: DbConn,
}

#[async_trait]
impl WalletRepositoryTrait for WalletRepository {
    async fn create(&self, dto: CreateWalletDto) -> Result<Wallet, DbErr> {
        let entity = WalletActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(dto.user_id),
            balance: Set(Decimal::ZERO),
            created_at: Set(Utc::now().into()),
            currency: Set(dto.currency),
        };

        let wallet = entity.insert(&self.db_conn).await?;

        Ok(wallet)
    }
}
