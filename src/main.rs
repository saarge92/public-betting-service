use crate::api::user;
use crate::container::AppContainer;
use crate::repository::{UserRepository, UserRepositoryParameters};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use sea_orm::{Database, DbConn};
use std::env;
use std::sync::Arc;

pub mod api;
mod container;
mod domain;
pub mod operation;
pub mod repository;

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().ok();
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    unsafe {
        std::env::set_var("RUST_LOG", log_level);
    }
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL в .env не найден");
    let db_conn: DbConn = Database::connect(&db_url)
        .await
        .expect("Ошибка подключения к БД");

    let container = AppContainer::builder()
        .with_component_parameters::<UserRepository>(UserRepositoryParameters { db: db_conn })
        .build();

    let shared_container = Arc::new(container);
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());

    // 2. ЗАПИСЬ В ЛОГ: выводим сообщение о старте
    log::info!("Сервер успешно запущен на порту {}", server_port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::from(shared_container.clone()))
            .service(web::scope("/api").configure(user::routes::init_routes))
    })
    .bind(("127.0.0.1", server_port.parse::<u16>().unwrap()))
    .expect("Не удалось привязать порт к серверу")
    .run()
    .await
    .expect("Ошибка вовремя запуска сервера");
}
