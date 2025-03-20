use serde::{Serialize, Deserialize};
use validator::Validate;

/// DTO для создания новой задачи.
///
/// Используется при создании задачи через POST-запрос.
///
/// - `title` — заголовок задачи (обязательное поле, от 3 до 100 символов).
/// - `description` — описание задачи (необязательное поле, до 500 символов).
#[derive(Serialize, Deserialize, Validate)]
pub struct TaskCreateDto {
    #[validate(length(
        min = 3,
        max = 100,
        message = "Title must be between 3 and 100 characters"
    ))]
    pub title: String,
    #[validate(length(
        max = 500,
        message = "Description must not exceed 500 characters"
    ))]
    pub description: Option<String>,
}

/// DTO для представления задачи в ответе от сервера.
///
/// Используется при получении списка задач или конкретной задачи.
///
/// - `id` — уникальный идентификатор задачи.
/// - `title` — заголовок задачи.
/// - `description` — описание задачи (может быть `None`).
/// - `user_id` — ID пользователя, владельца задачи.
#[derive(Serialize, Deserialize)]
pub struct TaskReadDto {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
}
