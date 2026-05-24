use crate::api::Inject;
use crate::api::middlewares::auth::Claims;
use crate::api::wallet::controller::WalletController;
use crate::domain::AppError;
use crate::operation::wallet::create::dto::CreateWalletDto;
use actix_web::web::Json;
use actix_web::{HttpResponse, web};
use uuid::Uuid;

pub async fn create_wallet_route(
    wallet_controller: Inject<WalletController>,
    payload: Json<CreateWalletDto>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Internal("Неверный формат UUID".to_string()))?;
    wallet_controller.create(user_id, payload).await
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/wallets").route("", web::post().to(create_wallet_route)));
}
