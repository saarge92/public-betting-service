use std::error::Error;
use crate::domain::AppError;
use crate::operation::user::{RegisterUserDto, UserServiceTrait};
use actix_web::{HttpResponse, web};
use shaku::{Provider};
use std::sync::Arc;
use crate::container::AppContainer;

pub struct UserHandler {
    user_service: Arc<dyn UserServiceTrait>,
}

impl UserHandler {
    pub fn new(user_service: Arc<dyn UserServiceTrait>) -> Self {
        Self { user_service }
    }

    pub async fn register(
        &self,
        request: web::Json<RegisterUserDto>,
    ) -> Result<HttpResponse, AppError> {
        let dto = request.into_inner();
        self.user_service.register(dto).await?;

        Ok(HttpResponse::Ok().finish())
    }
}

impl Provider<AppContainer> for UserHandler {
    type Interface = UserHandler;

    fn provide(container: &AppContainer) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        use shaku::HasComponent;

        let user_service: Arc<dyn UserServiceTrait> = container.resolve();

        Ok(Box::new(UserHandler::new(user_service)))
    }
}