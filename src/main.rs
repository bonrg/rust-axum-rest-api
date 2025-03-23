use std::sync::Arc;
use crate::db::db as other_db;
use crate::settings::settings as other_settings;
use crate::db::db::DatabaseTrait;
use hyper::Server;

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

    // Запуск сервера Axum
    Server::bind(&host.parse().unwrap())
        .serve(routes::root::routes(Arc::new(connection)))
        .await
        .unwrap_or_else(|e| panic!("❌ Server error: {}", e));
}

