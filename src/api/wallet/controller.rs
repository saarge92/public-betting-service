use crate::api::wallet::response::WalletResponseDto;
use crate::container::AppContainer;
use crate::domain::AppError;
use crate::operation::wallet::create::dto::CreateWalletDto;
use crate::operation::wallet::create::service::CreateWalletServiceTrait;
use crate::operation::wallet::list::service::WalletListServiceTrait;
use actix_web::{HttpResponse, web};
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct WalletController {
    create_wallet_service: Arc<dyn CreateWalletServiceTrait>,
    list_wallet_service: Arc<dyn WalletListServiceTrait>,
}

impl WalletController {
    pub fn new(
        create_wallet_service: Arc<dyn CreateWalletServiceTrait>,
        list_wallet_service: Arc<dyn WalletListServiceTrait>,
    ) -> WalletController {
        Self {
            create_wallet_service,
            list_wallet_service,
        }
    }

    pub async fn create(
        &self,
        current_user_id: Uuid,
        request: web::Json<CreateWalletDto>,
    ) -> Result<HttpResponse, AppError> {
        let dto = request.into_inner();
        if let Err(errors) = dto.validate() {
            return Err(AppError::Validation(errors.to_string()));
        }

        let wallet = self
            .create_wallet_service
            .create(dto, current_user_id)
            .await?;
        Ok(HttpResponse::Ok().json(WalletResponseDto::from(wallet)))
    }

    pub async fn list(&self, current_user_id: Uuid) -> Result<HttpResponse, AppError> {
        let wallets = self
            .list_wallet_service
            .list_by_user_id(current_user_id)
            .await?;
        let wallets_dto: Vec<WalletResponseDto> = wallets
            .into_iter()
            .map(WalletResponseDto::from)
            .collect();
        Ok(HttpResponse::Ok().json(wallets_dto))
    }
}

impl Provider<AppContainer> for WalletController {
    type Interface = WalletController;

    fn provide(container: &AppContainer) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        use shaku::HasComponent;

        let create_wallet_service: Arc<dyn CreateWalletServiceTrait> = container.resolve();
        let list_wallet_service: Arc<dyn WalletListServiceTrait> = container.resolve();

        Ok(Box::new(WalletController::new(
            create_wallet_service,
            list_wallet_service,
        )))
    }
}
