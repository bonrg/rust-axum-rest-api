use crate::db::db::{Database, DatabaseTrait};
use crate::dto::user::{UserReadDto, UserRegisterDto};
use crate::entities::user::User;
use crate::errors::api::ApiError;
use crate::errors::db::DbError;
use crate::errors::user::UserError;
use crate::repositories::user::{UserRepository, UserRepositoryTrait};
use sqlx::{query_as, Error as SqlxError};
use std::sync::Arc;

/// Сервис работы с пользователями (`UserService`).
///
/// Содержит бизнес-логику регистрации, валидации и обработки ошибок.
/// Использует `UserRepository` и соединение с базой данных.
#[derive(Clone)]
pub struct UserService {
    /// `user_repo` — репозиторий пользователей.
    user_repo: UserRepository,

    /// `db_conn` — подключение к базе данных.
    db_conn: Arc<Database>,
}

impl UserService {
    /// Создание нового экземпляра `UserService`.
    ///
    /// :param db_conn: Подключение к базе данных (`Arc<Database>`).
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
            db_conn: Arc::clone(db_conn),
        }
    }

    /// Создание нового пользователя.
    ///
    /// - Проверяет наличие пользователя по email.
    /// - Хеширует пароль.
    /// - Сохраняет пользователя в базу данных.
    ///
    /// :param payload: данные регистрации пользователя.
    /// :return: DTO созданного пользователя или ошибка (`ApiError`).
    pub async fn create_user(&self, payload: UserRegisterDto) -> Result<UserReadDto, ApiError> {
        match self.user_repo.find_by_email(payload.email.to_owned()).await {
            Some(_) => Err(UserError::UserAlreadyExists.into()),
            None => {
                let user = self.add_user(payload).await;

                match user {
                    Ok(user) => Ok(UserReadDto::from(user)),
                    Err(e) => match e {
                        SqlxError::Database(e) => match e.code() {
                            Some(code) if code == "23505" => {
                                Err(DbError::UniqueConstraintViolation(e.to_string()).into())
                            }
                            _ => Err(DbError::SomethingWentWrong(e.to_string()).into()),
                        },
                        _ => Err(DbError::SomethingWentWrong(e.to_string()).into()),
                    },
                }
            }
        }
    }

    /// Добавление нового пользователя в базу данных.
    ///
    /// Выполняет SQL-запрос с `RETURNING *`, возвращает созданного пользователя.
    ///
    /// :param payload: регистрационные данные.
    /// :return: модель пользователя из базы.
    async fn add_user(&self, payload: UserRegisterDto) -> Result<User, SqlxError> {
        let hashed_password = bcrypt::hash(payload.password, 4).unwrap();

        let user = query_as!(
            User,
            r#"
            INSERT INTO users (first_name, last_name, user_name, email, password, is_active)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, first_name, last_name, user_name, email, password, created_at, updated_at, is_active
            "#,
            payload.first_name,
            payload.last_name,
            payload.user_name,
            payload.email,
            hashed_password,
            1i8
        )
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(user)
    }

    /// Проверка пароля пользователя.
    ///
    /// :param user: модель пользователя из базы.
    /// :param password: переданный пароль в `login`.
    /// :return: `true` — если пароль корректен.
    pub fn verify_password(&self, user: &User, password: &str) -> bool {
        bcrypt::verify(password, &user.password).unwrap_or(false)
    }
}
