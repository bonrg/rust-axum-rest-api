use crate::response::api_response::ApiErrorResponse;
use async_trait::async_trait;
use axum::{
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// Ошибки, возникающие при парсинге и валидации входящего запроса.
///
/// - `ValidationError` — входные данные прошли JSON-десериализацию, но не прошли валидацию.
/// - `JsonRejection` — тело запроса содержит некорректный JSON.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

/// Специальный обёрточный тип `ValidatedRequest<T>`
///
/// Используется в Axum-хендлерах как extractor с автоматической JSON-десериализацией и валидацией.
///
/// Если валидация не проходит, возвращается ошибка `RequestError::ValidationError`.
/// Если JSON невалидный — `RequestError::JsonRejection`.
///
/// # Пример использования:
/// ```rust
/// async fn handler(ValidatedRequest(dto): ValidatedRequest<MyDto>) { ... }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = RequestError;

    /// Извлекает тело запроса и выполняет валидацию структуры `T`.
    ///
    /// **<u>:param req</u>**: HTTP-запрос.
    /// **<u>:param state</u>**: Объект состояния приложения.
    /// **<u>:return</u>**: Обёртка `ValidatedRequest<T>` при успехе, либо `RequestError` при ошибке.
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedRequest(value))
    }
}

/// Реализация конверсии `RequestError` в HTTP-ответ.
///
/// - `ValidationError` → 400 Bad Request + сообщение с деталями.
/// - `JsonRejection` → 400 Bad Request + сообщение о парсинге JSON.
impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        match self {
            RequestError::ValidationError(_) => {
                ApiErrorResponse::send(400, Some(self.to_string().replace('\n', ", ")))
            }
            RequestError::JsonRejection(_) => {
                ApiErrorResponse::send(400, Some(self.to_string()))
            }
        }
    }
}
