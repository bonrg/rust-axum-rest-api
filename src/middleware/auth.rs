use crate::errors::{api::ApiError, token::TokenError, user::UserError};
use crate::repositories::user::UserRepositoryTrait;
use crate::services::token::TokenServiceTrait;
use crate::states::user::TokenState;

use axum::{
    extract::State,
    http::{self, Request},
    middleware::Next,
    response::IntoResponse,
    body::Body,
};
use headers::Authorization;
use headers::authorization::Bearer;
use headers::Header;
use jsonwebtoken::errors::ErrorKind;

/// Middleware-проверка авторизации (`auth`).
///
/// Проверяет наличие JWT-токена в заголовке `Authorization: Bearer <token>`,
/// валидирует токен и извлекает пользователя из базы по email в claims.
///
/// - Если токен отсутствует — `TokenError::MissingToken`
/// - Если токен истёк — `TokenError::TokenExpired`
/// - Если токен некорректен — `TokenError::InvalidToken`
/// - Если пользователь не найден — `UserError::UserNotFound`
///
/// При успешной проверке пользователь добавляется в `Request.extensions()`.
pub async fn auth(
    State(state): State<TokenState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(name, value)| {
            if name == axum::http::header::AUTHORIZATION {
                Some(value)
            } else {
                None
            }
        });

    // Парсинг Bearer-токена
    let header: Authorization<Bearer> =
        Authorization::decode(&mut headers).map_err(|_| TokenError::MissingToken)?;

    let token = header.token();

    // Декодирование токена и получение claims
    match state.token_service.retrieve_token_claims(token) {
        Ok(token_data) => {
            // Поиск пользователя по email из claims
            let user = state.user_repo.find_by_email(token_data.claims.email).await;

            match user {
                Some(user) => {
                    req.extensions_mut().insert(user); // Добавление user в request
                    Ok(next.run(req).await)
                }
                None => Err(UserError::UserNotFound.into()),
            }
        }
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(TokenError::TokenExpired.into()),
            _ => Err(TokenError::InvalidToken(token.to_string()).into()),
        },
    }
}
