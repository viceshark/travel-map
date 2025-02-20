use std::sync::{Arc, Mutex};
use crate::models::Route;

// Преобразуем AppState в потокобезопасную структуру, используя Arc и Mutex
pub struct AppState {
    pub routes: Mutex<Vec<Route>>,  // Структура Mutex для безопасного доступа
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            routes: Mutex::new(Vec::new()),  // Изначально пустой список маршрутов
        }
    }
}

