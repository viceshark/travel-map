use actix_web::{get, web, HttpResponse, Responder};
use crate::models::Route;
use crate::state::AppState;

pub async fn get_routes(state: web::Data<AppState>) -> HttpResponse {
    let routes = state.routes.lock().unwrap(); // Получаем блокировку для безопасного доступа
    HttpResponse::Ok().json(&*routes) // Возвращаем список маршрутов в формате JSON
}

pub async fn create_route(
    state: web::Data<AppState>,
    route: web::Json<Route>, // Принимаем маршрут в теле запроса
) -> HttpResponse {
    let mut routes = state.routes.lock().unwrap(); // Блокируем список маршрутов
    let new_id = routes.len() + 1; // Генерируем новый ID для маршрута
    let new_route = Route::new(new_id, route.start.clone(), route.end.clone());
    routes.push(new_route.clone()); // Добавляем новый маршрут в список
    HttpResponse::Created().json(&new_route) // Отправляем созданный маршрут в ответ
}


#[get("/")]
async fn index() -> impl Responder {
    "Привет, путешественник! 🚀"
}