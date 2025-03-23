-- 0001_create_users.sql

-- Создание таблицы пользователей
CREATE TABLE IF NOT EXISTS users (
                                     id SERIAL PRIMARY KEY,
                                     first_name VARCHAR(255),
    last_name VARCHAR(255),
    user_name VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT FALSE
    );
