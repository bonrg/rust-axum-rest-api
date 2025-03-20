use serde::{Deserialize, Serialize};

/// DTO для представления JWT-токена (например, при ответе клиенту).
///
/// Обычно возвращается после успешной авторизации или регистрации.
///
/// - `token` — строка токена (JWT).
/// - `iat` — время выпуска токена (issued at, Unix timestamp).
/// - `exp` — срок действия токена (expiration time, Unix timestamp).
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenReadDto {
    pub token: String,
    pub iat: i64,
    pub exp: i64,
}

/// DTO для представления **payload (claims)** токена.
///
/// Используется при валидации или декодировании токена на сервере.
///
/// - `sub` — ID пользователя (subject).
/// - `email` — Email пользователя.
/// - `iat` — Время выпуска токена.
/// - `exp` — Время истечения срока действия токена.
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenClaimsDto {
    pub sub: i32,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}
