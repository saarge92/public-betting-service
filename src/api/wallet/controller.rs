use crate::api::wallet::response::WalletResponseDto;
use crate::container::AppContainer;
use crate::domain::AppError;
use crate::operation::wallet::create::dto::CreateWalletDto;
use crate::operation::wallet::create::wallet_service::CreateWalletServiceTrait;
use actix_web::{HttpResponse, web};
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

pub struct WalletController {
    create_wallet_service: Arc<dyn CreateWalletServiceTrait>,
}

impl WalletController {
    pub fn new(create_wallet_service: Arc<dyn CreateWalletServiceTrait>) -> WalletController {
        Self {
            create_wallet_service,
        }
    }

    pub async fn create(
        &self,
        current_user_id: Uuid,
        request: web::Json<CreateWalletDto>,
    ) -> Result<HttpResponse, AppError> {
        let wallet = self
            .create_wallet_service
            .create(request.into_inner(), current_user_id)
            .await?;
        Ok(HttpResponse::Ok().json(WalletResponseDto::from(wallet)))
    }
}

impl Provider<AppContainer> for WalletController {
    type Interface = WalletController;

    fn provide(container: &AppContainer) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        use shaku::HasComponent;

        let create_wallet_service: Arc<dyn CreateWalletServiceTrait> = container.resolve();

        Ok(Box::new(WalletController::new(create_wallet_service)))
    }
}
