use crate::settings::settings;
use crate::dto::token::{TokenClaimsDto, TokenReadDto};
use crate::entities::user::User;
use crate::errors::token::TokenError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

/// Сервис работы с JWT-токенами (`TokenService`).
///
/// Обеспечивает генерацию и валидацию токенов.
///
/// - `secret` — секретный ключ для подписи токенов.
#[derive(Clone)]
pub struct TokenService {
    secret: String,
}

/// Интерфейс `TokenServiceTrait`.
///
/// Определяет базовые методы генерации и валидации токенов.
///
/// - `new` — создание сервиса.
/// - `generate_token` — генерация токена по пользователю.
/// - `retrieve_token_claims` — декодирование и проверка токена.
/// - `TOKEN_EXPIRATION` — срок действия токена в минутах.
pub trait TokenServiceTrait {
    /// Создание экземпляра `TokenService`.
    fn new() -> Self;

    /// Декодирование и проверка токена.
    ///
    /// :param token: JWT-токен в виде строки.
    /// :return: Расшифрованные claims (`TokenClaimsDto`) или ошибка.
    fn retrieve_token_claims(
        &self,
        token: &str,
    ) -> jsonwebtoken::errors::Result<TokenData<TokenClaimsDto>>;

    /// Генерация нового JWT-токена для пользователя.
    ///
    /// :param user: сущность пользователя.
    /// :return: `TokenReadDto` (токен + время iat/exp) или ошибка.
    fn generate_token(&self, user: User) -> Result<TokenReadDto, TokenError>;

    const TOKEN_EXPIRATION: i64;
}

impl TokenServiceTrait for TokenService {
    /// Создание `TokenService` — читает JWT-секрет из окружения.
    fn new() -> Self {
        Self {
            secret: settings::get("JWT_SECRET"),
        }
    }

    /// Декодирует и валидирует токен.
    fn retrieve_token_claims(
        &self,
        token: &str,
    ) -> jsonwebtoken::errors::Result<TokenData<TokenClaimsDto>> {
        decode::<TokenClaimsDto>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )
    }

    /// Генерирует JWT-токен с заданным временем жизни.
    fn generate_token(&self, user: User) -> Result<TokenReadDto, TokenError> {
        let iat = Utc::now().timestamp();
        let exp = Utc::now()
            .checked_add_signed(Duration::minutes(Self::TOKEN_EXPIRATION))
            .unwrap()
            .timestamp();

        let claims = TokenClaimsDto {
            sub: user.id,
            email: user.email,
            iat,
            exp,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
            .map_err(|e| TokenError::TokenCreationError(e.to_string()))?;

        Ok(TokenReadDto { token, iat, exp })
    }

    /// Время жизни токена: 30 минут.
    const TOKEN_EXPIRATION: i64 = 30;
}
