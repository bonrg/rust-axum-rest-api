use crate::errors::{db::DbError, token::TokenError, user::UserError};
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// Универсальное перечисление ошибок API.
///
/// Служит в качестве централизованной оболочки для всех типов ошибок в приложении.
///
/// - `TokenError` — ошибки, связанные с JWT-токенами.
/// - `UserError` — ошибки, связанные с пользователями.
/// - `DbError` — ошибки при работе с базой данных.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DbError(#[from] DbError),
}

/// Реализация преобразования `ApiError` в HTTP-ответ.
///
/// Все вложенные ошибки автоматически преобразуются через `IntoResponse` соответствующих типов.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TokenError(error) => error.into_response(),
            ApiError::UserError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
        }
    }
}
