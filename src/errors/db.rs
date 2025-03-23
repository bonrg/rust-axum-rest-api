use crate::response::api::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Ошибки, возникающие при работе с базой данных.
///
/// Используется для обработки ошибок, связанных с операциями в СУБД.
///
/// - `SomethingWentWrong` — внутренняя ошибка при выполнении запроса.
/// - `UniqueConstraintViolation` — нарушение уникального ограничения (дубликат записи).
#[derive(Error, Debug)]
pub enum DbError {
    #[error("{0}")]
    SomethingWentWrong(String),
    #[error("Duplicate entry exists")]
    UniqueConstraintViolation(String),
}

/// Реализация преобразования `DbError` в HTTP-ответ.
///
/// - `SomethingWentWrong` → 500 Internal Server Error
/// - `UniqueConstraintViolation` → 409 Conflict
impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DbError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::UniqueConstraintViolation(_) => StatusCode::CONFLICT,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
