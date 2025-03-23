use crate::handlers::user;
use crate::states::user::UserState;
use axum::{routing::post, Router};

/// Маршруты регистрации пользователя (`/register`).
///
/// - `POST /register` — регистрация нового пользователя.
/// Используется `UserState` как shared state.
pub fn routes() -> Router<UserState> {
    Router::new().route("/register", post(user::register_user))
}
