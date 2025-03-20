use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Перечисление ошибок, связанных с пользователем.
///
/// Эти ошибки могут возникнуть при выполнении операций авторизации, регистрации или получения пользователя.
///
/// - `UserNotFound` — пользователь не найден.
/// - `UserAlreadyExists` — пользователь с таким email или username уже существует.
/// - `InvalidPassword` — введён неверный пароль.
#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid password")]
    InvalidPassword,
}

/// Реализация преобразования `UserError` в HTTP-ответ.
///
/// Ошибка будет автоматически преобразована в JSON-ответ с соответствующим статусом:
///
/// - `UserNotFound` → 404 Not Found
/// - `UserAlreadyExists` → 400 Bad Request
/// - `InvalidPassword` → 400 Bad Request
impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::UserAlreadyExists => StatusCode::BAD_REQUEST,
            UserError::InvalidPassword => StatusCode::BAD_REQUEST,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
