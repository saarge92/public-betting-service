use uuid::Uuid;

pub struct CreateWalletDto {
    pub user_id: Uuid,
    pub currency: String,
}
