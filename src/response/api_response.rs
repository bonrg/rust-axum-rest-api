use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

/// Универсальный формат успешного ответа от API.
///
/// Оборачивает полезную нагрузку (`data`) в единый ответ.
///
/// - `data` — полезные данные (результат выполнения запроса).
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApiSuccessResponse<T: Serialize> {
    data: T,
}

/// Унифицированный формат ответа при ошибке.
///
/// Позволяет вернуть код ошибки и человекочитаемое сообщение.
///
/// - `status` — HTTP статус-код ошибки.
/// - `message` — текстовое описание ошибки (может быть None).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiErrorResponse {
    pub message: Option<String>,
    #[serde(rename = "code")]
    pub status: u16,
}

impl<T: Serialize> ApiSuccessResponse<T> {
    /// Создаёт успешный ответ с переданными данными.
    ///
    /// **<u>:param data</u>**: данные, которые нужно вернуть клиенту.
    /// **<u>:return</u>**: структура `ApiSuccessResponse<T>`.
    pub(crate) fn send(data: T) -> Self {
        ApiSuccessResponse { data }
    }
}

impl ApiErrorResponse {
    /// Создаёт и отправляет стандартный JSON-ответ об ошибке.
    ///
    /// **<u>:param status</u>**: HTTP статус ошибки.
    /// **<u>:param message</u>**: сообщение об ошибке (опционально).
    /// **<u>:return</u>**: HTTP-ответ (`axum::Response`) с JSON телом.
    pub(crate) fn send(status: u16, message: Option<String>) -> Response {
        ApiErrorResponse { message, status }.into_response()
    }
}

impl IntoResponse for ApiErrorResponse {
    /// Конвертация `ApiErrorResponse` в HTTP-ответ.
    ///
    /// Возвращается JSON с кодом и сообщением.
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}
