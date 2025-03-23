use crate::response::api::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Ошибки, связанные с задачами (`TaskError`).
///
/// Используются при обработке CRUD-операций с задачами.
///
/// - `TaskNotFound` — задача не найдена.
/// - `TaskAlreadyExists` — задача с таким названием уже существует.
/// - `ForbiddenTaskAccess` — попытка доступа к задаче, которая принадлежит другому пользователю.
#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task not found")]
    TaskNotFound,
    #[error("Task already exists")]
    TaskAlreadyExists,
    #[error("Access to this task is forbidden")]
    ForbiddenTaskAccess,
}

/// Реализация преобразования `TaskError` в HTTP-ответ.
///
/// - `TaskNotFound` → 404 Not Found
/// - `TaskAlreadyExists` → 400 Bad Request
/// - `ForbiddenTaskAccess` → 403 Forbidden
impl IntoResponse for TaskError {
    fn into_response(self) -> Response {
        let status_code = match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskAlreadyExists => StatusCode::BAD_REQUEST,
            TaskError::ForbiddenTaskAccess => StatusCode::FORBIDDEN,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
