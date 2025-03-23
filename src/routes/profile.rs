use crate::handlers::user;
use axum::{routing::get, Router};

/// Маршруты профиля пользователя (`/profile`).
///
/// - `GET /profile` — получить данные текущего авторизованного пользователя.
pub fn routes() -> Router {
    Router::new().route("/profile", get(user::get_profile))
}
