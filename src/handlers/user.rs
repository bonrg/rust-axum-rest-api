use crate::dto::{token::TokenReadDto, user::{UserLoginDto, UserReadDto, UserRegisterDto}};
use crate::errors::{api::ApiError, request::ValidatedRequest, user::UserError};
use crate::repositories::user::UserRepositoryTrait;
use crate::services::token::TokenServiceTrait;
use crate::states::user::{AuthState, UserState};
use crate::entities::user::User;
use crate::response::api::ApiSuccessResponse;
use axum::{extract::State, Json, Extension};

/// Обработчик авторизации пользователя.
///
/// Проверяет наличие пользователя по email, сверяет пароль и выдаёт JWT токен.
///
/// - `payload` — данные для входа: email и пароль.
/// - `user_repo` — используется для поиска пользователя по email.
/// - `user_service` — выполняет проверку пароля.
/// - `token_service` — генерирует JWT токен.
///
/// Возвращает:
/// - `TokenReadDto` при успешной авторизации;
/// - Ошибку `UserNotFound` или `InvalidPassword`, если данные неверные.
pub async fn auth(
    State(state): State<AuthState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginDto>,
) -> Result<Json<TokenReadDto>, ApiError> {
    // Поиск пользователя по email
    let user = state
        .user_repo
        .find_by_email(payload.email)
        .await
        .ok_or(UserError::UserNotFound)?;

    // Проверка пароля
    match state.user_service.verify_password(&user, &payload.password) {
        true => {
            // Генерация токена
            let token = state.token_service.generate_token(user)?;
            Ok(Json(token))
        }
        false => Err(UserError::InvalidPassword)?,
    }
}

/// Обработчик получения профиля текущего пользователя.
///
/// Используется для получения данных авторизованного пользователя.
///
/// - `current_user` — извлекается из JWT-токена через middleware.
/// - Возвращает `UserReadDto` в формате `ApiSuccessResponse`.
///
/// # Пример ответа:
/// ```json
/// {
///   "data": {
///     "id": 1,
///     "user_name": "johndoe",
///     "email": "john@example.com",
///     ...
///   }
/// }
/// ```
pub async fn get_profile(
    Extension(current_user): Extension<User>,
) -> Json<ApiSuccessResponse<UserReadDto>> {
    Json(ApiSuccessResponse::send(UserReadDto::from(current_user)))
}

/// Обработчик регистрации нового пользователя.
///
/// Выполняет создание пользователя в системе на основе регистрационных данных.
///
/// - `payload` — регистрационные данные пользователя.
/// - `state.user_service` — создаёт пользователя в базе.
///
/// Возвращает:
/// - `UserReadDto` — данные зарегистрированного пользователя;
/// - `ApiError` — в случае ошибки валидации или дубликата email/username.
pub async fn register_user(
    State(state): State<UserState>,
    ValidatedRequest(payload): ValidatedRequest<UserRegisterDto>,
) -> Result<Json<ApiSuccessResponse<UserReadDto>>, ApiError> {
    let user = state.user_service.create_user(payload).await?;
    Ok(Json(ApiSuccessResponse::send(user)))
}