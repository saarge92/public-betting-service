use crate::api::{Inject, UserHandler};
use crate::domain::AppError;
use crate::operation::user::RegisterUserDto;
use actix_web::{HttpResponse, web};

pub async fn register_user(
    handler: Inject<UserHandler>,
    request: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, AppError> {
    handler.register(request).await
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").route("/register", web::post().to(register_user)));
}
