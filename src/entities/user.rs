use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// Модель пользователя, соответствующая таблице в базе данных.
///
/// Используется для хранения информации о пользователе в базе данных.
///
/// - `id` — уникальный идентификатор пользователя.
/// - `first_name` — имя пользователя (может отсутствовать).
/// - `last_name` — фамилия пользователя (может отсутствовать).
/// - `user_name` — уникальное имя пользователя (username).
/// - `email` — адрес электронной почты пользователя.
/// - `password` — хеш пароля пользователя.
/// - `created_at` — дата создания пользователя.
/// - `updated_at` — дата последнего обновления (может отсутствовать).
/// - `is_active` — статус активности пользователя (1 — активен, 0 — неактивен).
#[derive(Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_active: i32,
}
