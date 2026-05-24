use crate::domain::wallet;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct WalletResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: String,
    pub currency: String,
    pub created_at: String,
}

impl From<wallet::Model> for WalletResponseDto {
    fn from(model: wallet::Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            balance: model.balance.to_string(),
            currency: model.currency,
            created_at: model.created_at.to_rfc3339(),
        }
    }
}
