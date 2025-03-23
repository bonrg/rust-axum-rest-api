use crate::handlers::user;
use crate::states::user::AuthState;
use axum::{routing::post, Router};

/// Маршруты для аутентификации (`/auth`).
///
/// Используется `AuthState` как shared state.
///
/// - `POST /auth` — авторизация (логин).
pub fn routes() -> Router<AuthState> {
    Router::new().route("/auth", post(user::auth))
}
