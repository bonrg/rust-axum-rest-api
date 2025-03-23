use crate::db::db::{Database, DatabaseTrait};
use crate::entities::user::User;
use async_trait::async_trait;
use sqlx::{Error};
use std::sync::Arc;

/// Репозиторий пользователей (`UserRepository`).
///
/// Предоставляет методы доступа к таблице пользователей в базе данных.
#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

/// Трейт `UserRepositoryTrait` — интерфейс репозитория пользователей.
///
/// Определяет базовые методы работы с таблицей пользователей.
///
/// - `new` — создание экземпляра репозитория.
/// - `find_by_email` — поиск пользователя по email.
/// - `find` — поиск пользователя по ID.
#[async_trait]
pub trait UserRepositoryTrait {
    /// Создание нового экземпляра репозитория пользователей.
    ///
    /// :param db_conn: подключение к базе данных.
    fn new(db_conn: &Arc<Database>) -> Self;

    /// Поиск пользователя по email.
    ///
    /// :param email: адрес электронной почты пользователя.
    /// :return: `Some(User)`, если пользователь найден, иначе `None`.
    async fn find_by_email(&self, email: String) -> Option<User>;

    /// Поиск пользователя по ID.
    ///
    /// :param id: идентификатор пользователя.
    /// :return: `User`, если найден, либо `sqlx::Error`.
    async fn find(&self, id: u64) -> Result<User, Error>;
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn find_by_email(&self, email: String) -> Option<User> {
        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await;

        result.unwrap_or(None)
    }

    async fn find(&self, id: u64) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1" //
        )
            .bind(id as i64) // или напрямую, если id: i64
            .fetch_one(self.db_conn.get_pool())
            .await
    }
}
