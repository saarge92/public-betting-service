pub mod middlewares;
pub mod user;
pub mod wallet;

use crate::container::AppContainer;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest, web};
use shaku::HasProvider;
use std::future::{Ready, ready};
use std::ops::Deref;
pub use user::*;

pub struct Inject<T>(Box<T>);

impl<T> Deref for Inject<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> FromRequest for Inject<T>
where
    AppContainer: HasProvider<T>,
    T: 'static,
{
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        if let Some(container) = req.app_data::<web::Data<AppContainer>>() {
            match HasProvider::provide(container.get_ref()) {
                Ok(handler) => ready(Ok(Inject(handler))),
                Err(e) => {
                    log::error!("Ошибка сборки зависимости в DI контейнере: {:?}", e);
                    // Возвращаем 500 ошибку, если контейнер не смог собраться
                    ready(Err(actix_web::error::ErrorInternalServerError(
                        "DI compilation error",
                    )))
                }
            }
        } else {
            log::error!("AppContainer не найден в app_data. Проверь main.rs!");
            ready(Err(actix_web::error::ErrorInternalServerError(
                "DI container missing",
            )))
        }
    }
}
