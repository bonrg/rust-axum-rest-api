use crate::settings::settings;
use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres, PgPool};

/// Структура `Database` содержит пул подключений к базе данных PostgreSQL.
pub struct Database {
    pool: Pool<Postgres>,
}

/// Трейт `DatabaseTrait` определяет поведение для инициализации и получения пула подключений.
///
/// Этот трейт можно реализовать для любой структуры, которая инкапсулирует подключение к базе данных.
#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, Error>
    where
        Self: Sized;

    fn get_pool(&self) -> &Pool<Postgres>;
}

/// Реализация трейта `DatabaseTrait` для структуры `Database`.
#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, Error> {
        let database_url = settings::get("DATABASE_URL");
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
