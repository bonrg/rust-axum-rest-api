use std::sync::Arc;
use crate::db::db as other_db;
use crate::settings::settings as other_settings;
use crate::db::db::DatabaseTrait;
use tokio::net::TcpListener;

mod settings;
mod db;
mod dto;
mod entities;
mod errors;
mod response;
mod handlers;
mod states;
mod repositories;
mod services;
mod middleware;
mod routes;

#[tokio::main]
async fn main() {
    // Инициализация переменных окружения (.env)
    other_settings::init();

    // Инициализация подключения к базе данных
    let connection = other_db::Database::init()
        .await
        .unwrap_or_else(|e| panic!("❌ Database error: {}", e));

    // Чтение порта из .env
    let host = format!("0.0.0.0:{}", 3000);

    // Инициализация логгера (tracing)
    tracing_subscriber::fmt::init();
    println!("🚀 Server running on http://{}", host);

    let listener = TcpListener::bind(&host)
        .await
        .expect("Failed to bind address");

    // Инициализируем маршруты
    let app = crate::routes::root::routes(Arc::new(connection));

    // Запускаем сервер с axum::serve
    axum::serve(listener, app)
        .await
        .expect("Server error");
}

