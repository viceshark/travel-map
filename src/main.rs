mod state;
mod handlers;
mod models;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use handlers::{get_routes, create_route};
use crate::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Инициализация логирования

    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Передаем состояние в приложение
            .wrap(Logger::default()) // Добавляем логирование
            .route("/routes", web::get().to(get_routes))  // GET /routes
            .route("/routes", web::post().to(create_route)) // POST /routes
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}