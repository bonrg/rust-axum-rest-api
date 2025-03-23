use crate::db::db::Database;
use crate::repositories::user::{UserRepositoryTrait, UserRepository};
use crate::services::token::{TokenService, TokenServiceTrait};
use crate::services::user::UserService;
use std::sync::Arc;

/// Состояние для модуля авторизации (`AuthState`).
///
/// Объединяет все зависимости, необходимые для работы handler'ов аутентификации и авторизации.
///
/// - `token_service` — сервис генерации и проверки JWT-токенов.
/// - `user_repo` — репозиторий для работы с пользователями.
/// - `user_service` — бизнес-логика работы с пользователями.
#[derive(Clone)]
pub struct AuthState {
    pub(crate) token_service: TokenService,
    pub(crate) user_repo: UserRepository,
    pub(crate) user_service: UserService,
}

impl AuthState {
    /// Создаёт новый экземпляр `AuthState` на основе подключения к базе данных.
    ///
    /// :param db_conn: Обёртка над пулом подключения к базе (`Arc<Database>`).
    /// :return: Инициализированное состояние авторизации.
    pub fn new(db_conn: &Arc<Database>) -> AuthState {
        Self {
            token_service: TokenService::new(),
            user_service: UserService::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }
}

/// Состояние для пользовательского модуля (`UserState`).
///
/// Содержит зависимости, необходимые для работы handler'ов, связанных с пользователями.
///
/// - `user_service` — бизнес-логика пользователей.
/// - `user_repo` — репозиторий для работы с таблицей пользователей.
#[derive(Clone)]
pub struct UserState {
    pub user_service: UserService,
    pub user_repo: UserRepository,
}

impl UserState {
    /// Создаёт новый экземпляр `UserState`.
    ///
    /// :param db_conn: Обёртка над пулом подключения к базе (`Arc<Database>`).
    /// :return: Готовое состояние `UserState`.
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_service: UserService::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }
}

/// Состояние для работы с JWT-токенами (`TokenState`).
///
/// Используется для handler'ов, связанных с валидацией, обновлением или генерацией токенов.
///
/// - `token_service` — сервис генерации и декодирования JWT.
/// - `user_repo` — доступ к данным пользователей (например, для проверки при refresh).
#[derive(Clone)]
pub struct TokenState {
    pub token_service: TokenService,
    pub user_repo: UserRepository,
}

impl TokenState {
    /// Создаёт новый экземпляр `TokenState`.
    ///
    /// :param db_conn: Подключение к базе данных (`Arc<Database>`).
    /// :return: Инициализированное состояние `TokenState`.
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            token_service: TokenService::new(),
            user_repo: UserRepository::new(db_conn),
        }
    }
}
