use crate::api::{Inject, UserController};
use crate::domain::AppError;
use crate::operation::user::{LoginDto, RegisterUserDto};
use actix_web::{HttpResponse, web};

pub async fn register_user(
    handler: Inject<UserController>,
    request: web::Json<RegisterUserDto>,
) -> Result<HttpResponse, AppError> {
    handler.register(request).await
}

pub async fn login(
    handler: Inject<UserController>,
    request: web::Json<LoginDto>,
) -> Result<HttpResponse, AppError> {
    handler.login(request).await
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login)),
    );
}
