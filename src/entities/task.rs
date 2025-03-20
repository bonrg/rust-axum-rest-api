use serde::{Serialize, Deserialize};

/// Модель задачи, соответствующая таблице в базе данных.
///
/// Представляет задачу, привязанную к конкретному пользователю.
///
/// - `id` — уникальный идентификатор задачи.
/// - `title` — заголовок задачи.
/// - `description` — описание задачи (может отсутствовать).
/// - `user_id` — идентификатор пользователя, которому принадлежит задача.
#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
}
