use super::auth;
use crate::db::db::Database;
use crate::middleware::auth as auth_middleware;
use crate::routes::{profile, register};
use crate::states::user::{AuthState, TokenState, UserState};

use axum::{
    middleware,
    routing::{get, IntoMakeService},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Главный маршрутизатор приложения (`/api`).
///
/// Объединяет все маршруты:
/// - `/auth` — авторизация
/// - `/register` — регистрация
/// - `/profile` — защищённый маршрут, требует JWT
/// - `/health` — простой healthcheck
///
/// Использует отдельные `State` для модулей и middleware авторизации.
///
/// :param db_conn: подключение к базе данных
/// :return: готовый `IntoMakeService` для запуска приложения
pub fn routes(db_conn: Arc<Database>) -> Router {
    // Инициализация всех состояний
    let auth_state = AuthState::new(&db_conn);
    let user_state = UserState::new(&db_conn);
    let token_state = TokenState::new(&db_conn);

    // Объединение маршрутов
    let merged_router = auth::routes()
        .with_state(auth_state)
        .merge(register::routes().with_state(user_state))
        .merge(
            profile::routes().layer(
                ServiceBuilder::new()
                    .layer(middleware::from_fn_with_state(token_state, auth_middleware::auth)),
            ),
        )
        .merge(Router::new().route("/health", get(|| async { "Healthy..." })));

    // Финальный роутер с базовым префиксом `/api` и логгированием
    let app_router = Router::new()
        .nest("/api", merged_router)
        .layer(TraceLayer::new_for_http());

    app_router
}
