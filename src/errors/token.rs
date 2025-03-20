use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Перечисление ошибок, связанных с токенами авторизации (JWT).
///
/// Возникают при обработке, создании или валидации токенов.
///
/// - `InvalidToken` — токен повреждён или невалиден.
/// - `TokenExpired` — срок действия токена истёк.
/// - `MissingToken` — заголовок Authorization отсутствует или не содержит токен.
/// - `TokenCreationError` — ошибка при генерации нового токена.
#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Invalid token")]
    InvalidToken(String),
    #[error("Token has expired")]
    TokenExpired,
    #[error("Missing Bearer token")]
    MissingToken,
    #[error("Token error: {0}")]
    TokenCreationError(String),
}

/// Реализация конверсии `TokenError` в HTTP-ответ Axum.
///
/// Возвращается стандартный JSON-ответ с кодом и сообщением об ошибке:
///
/// - `InvalidToken` → 401 Unauthorized
/// - `TokenExpired` → 401 Unauthorized
/// - `MissingToken` → 401 Unauthorized
/// - `TokenCreationError` → 500 Internal Server Error
impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let status_code = match self {
            TokenError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            TokenError::TokenExpired => StatusCode::UNAUTHORIZED,
            TokenError::MissingToken => StatusCode::UNAUTHORIZED,
            TokenError::TokenCreationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
