use crate::container::AppContainer;
use crate::domain::AppError;
use crate::operation::user::auth_service::AuthUserServiceTrait;
use crate::operation::user::{LoginDto, RegisterUserDto, UserServiceTrait};
use actix_web::{HttpResponse, web};
use shaku::Provider;
use std::error::Error;
use std::sync::Arc;

pub struct UserHandler {
    user_service: Arc<dyn UserServiceTrait>,
    auth_service: Arc<dyn AuthUserServiceTrait>,
}

impl UserHandler {
    pub fn new(
        user_service: Arc<dyn UserServiceTrait>,
        auth_service: Arc<dyn AuthUserServiceTrait>,
    ) -> Self {
        Self {
            user_service,
            auth_service,
        }
    }

    pub async fn register(
        &self,
        request: web::Json<RegisterUserDto>,
    ) -> Result<HttpResponse, AppError> {
        let dto = request.into_inner();
        self.user_service.register(dto).await?;

        Ok(HttpResponse::Ok().finish())
    }

    pub async fn login(&self, request: web::Json<LoginDto>) -> Result<HttpResponse, AppError> {
        let dto = request.into_inner();

        let response = self.auth_service.login(dto).await?;

        Ok(HttpResponse::Ok().json(response))
    }
}

impl Provider<AppContainer> for UserHandler {
    type Interface = UserHandler;

    fn provide(container: &AppContainer) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        use shaku::HasComponent;

        let user_service: Arc<dyn UserServiceTrait> = container.resolve();
        let auth_service: Arc<dyn AuthUserServiceTrait> = container.resolve();

        Ok(Box::new(UserHandler::new(user_service, auth_service)))
    }
}
