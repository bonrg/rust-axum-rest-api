use crate::response::api::ApiErrorResponse;
use async_trait::async_trait;
use axum::{
    extract::{FromRequest, Request, Json},
    body::Body,
    response::{IntoResponse, Response},
    BoxError,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// Ошибки при парсинге и валидации запроса.
///
/// - `ValidationError` — JSON-десериализация успешна, но валидация не пройдена.
/// - `JsonParseError` — тело запроса содержит некорректный JSON.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("Invalid JSON payload: {0}")]
    JsonParseError(#[from] axum::extract::rejection::JsonRejection),
}

/// Обёртка `ValidatedRequest<T>` — валидируемый JSON-запрос.
///
/// Используется как `Json<T>`, но с автоматической валидацией.
///
/// - Если JSON корректен и проходит валидацию, передаётся в handler.
/// - Если JSON сломан → `JsonParseError` (400 Bad Request).
/// - Если JSON корректен, но не проходит валидацию → `ValidationError` (422 Unprocessable Entity).
///
/// # Пример использования:
/// ```rust
/// async fn handler(ValidatedRequest(dto): ValidatedRequest<MyDto>) { ... }
/// ```
#[derive(Debug)]
pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S, Body> for ValidatedRequest<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = Response;

    /// Парсинг JSON и валидация структуры `T`.
    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        // 1. Парсим JSON
        let Json(value) = Json::<T>::from_request(req, state).await.map_err(|err| {
            ApiErrorResponse::send(400, Some(format!("Invalid JSON: {}", err)))
        })?;

        // 2. Проверяем валидацию
        value.validate().map_err(|err| {
            ApiErrorResponse::send(422, Some(format!("Validation error: {}", err)))
        })?;

        Ok(ValidatedRequest(value))
    }
}

/// Реализация `IntoResponse` для `RequestError`.
///
/// - `ValidationError` → 422 Unprocessable Entity.
/// - `JsonParseError` → 400 Bad Request.
impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        match self {
            RequestError::ValidationError(err) => {
                ApiErrorResponse::send(422, Some(format!("Validation error: {}", err)))
            }
            RequestError::JsonParseError(err) => {
                ApiErrorResponse::send(400, Some(format!("Invalid JSON: {}", err)))
            }
        }
    }
}
