use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateWalletDto {
    pub currency: String,
}
