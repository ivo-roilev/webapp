#[cfg(not(test))]
use sqlx::mysql::{MySqlPool as Pool, MySqlPoolOptions as PoolOptions};
#[cfg(test)]
use sqlx::sqlite::{SqlitePool as Pool, SqlitePoolOptions as PoolOptions};

use sqlx::Row;
use std::time::Duration;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

// SQLite-compatible schema for testing
// Note: Uses AUTOINCREMENT (SQLite) instead of AUTO_INCREMENT (MySQL)
// and INTEGER PRIMARY KEY instead of INT AUTO_INCREMENT
#[cfg(test)]
const CREATE_SCHEMA_SQLITE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(16) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_profiles (
    user_id INTEGER PRIMARY KEY,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS user_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    parent_property VARCHAR(255),
    property VARCHAR(255) NOT NULL,
    value TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetadata {
    pub parent_property: Option<String>,
    pub property: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile: Option<UserProfile>,
    pub metadata: Vec<UserMetadata>,
}

#[derive(Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub profile: Option<UserProfile>,
    pub metadata: Vec<UserMetadata>,
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    #[error("Username already exists")]
    DuplicateUsername,
    #[error("User not found")]
    UserNotFound,
    #[error("Database error: {0}")]
    QueryError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::UserNotFound,
            sqlx::Error::Database(db_err) => {
                if db_err.message().contains("Duplicate entry") || db_err.message().contains("UNIQUE") {
                    DatabaseError::DuplicateUsername
                } else {
                    DatabaseError::QueryError(db_err.message().to_string())
                }
            }
            _ => DatabaseError::QueryError(err.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    /// Initialize database connection pool with environment variables
    pub async fn new() -> Result<Self, DatabaseError> {
        let database_url = std::env::var("DATABASE_URL")
            .or_else(|_| {
                let host = std::env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
                let port = std::env::var("DATABASE_PORT").unwrap_or_else(|_| "3306".to_string());
                let user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "root".to_string());
                let password = std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "".to_string());
                let database = std::env::var("DATABASE_NAME").unwrap_or_else(|_| "webapp_db".to_string());

                Ok(format!(
                    "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
                    user, password, host, port, database
                ))
            })
            .map_err(|e: sqlx::Error| DatabaseError::ConnectionError(e.to_string()))?;

        let pool = PoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&database_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Database { pool })
    }

    /// Create a test database with in-memory SQLite
    /// Used exclusively for integration tests - does not affect production
    #[cfg(test)]
    pub async fn new_test() -> Result<Self, DatabaseError> {
        // Create in-memory SQLite database
        let pool = PoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Initialize schema (running multiple statements in one query is supported by sqlx for sqlite)
        sqlx::query(CREATE_SCHEMA_SQLITE)
            .execute(&pool)
            .await
            .map_err(|e| DatabaseError::QueryError(format!("Failed to create schema: {}", e)))?;

        Ok(Database { pool })
    }

    /// Create a new user record in the database using multiple tables and transactions
    pub async fn create_user(&self, user: &CreateUserRequest) -> Result<i32, DatabaseError> {
        let mut tx = self.pool.begin().await.map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        // 1. Insert into core 'users' table
        let result = sqlx::query(
            "INSERT INTO users (username, password) VALUES (?, ?)"
        )
        .bind(&user.username)
        .bind(&user.password)
        .execute(&mut *tx)
        .await?;

        // Get the new user ID
        #[cfg(not(test))]
        let user_id = result.last_insert_id() as i64;
        #[cfg(test)]
        let user_id = result.last_insert_rowid();
        let user_id = user_id as i32;

        // 2. Insert into 'user_profiles' if profile data exists
        if let Some(ref profile) = user.profile {
            sqlx::query(
                "INSERT INTO user_profiles (user_id, first_name, last_name, email) VALUES (?, ?, ?, ?)"
            )
            .bind(user_id)
            .bind(&profile.first_name)
            .bind(&profile.last_name)
            .bind(&profile.email)
            .execute(&mut *tx)
            .await?;
        }

        // 3. Insert into 'user_metadata' for hobbies, titles, and extra metadata
        for meta in &user.metadata {
            sqlx::query(
                "INSERT INTO user_metadata (user_id, parent_property, property, value) VALUES (?, ?, ?, ?)"
            )
            .bind(user_id)
            .bind(&meta.parent_property)
            .bind(&meta.property)
            .bind(&meta.value)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await.map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(user_id)
    }

    /// Find user by username (optimized for authentication)
    pub async fn authenticate_user(&self, username: &str) -> Result<(i32, String), DatabaseError> {
        let row: (i32, String) = sqlx::query_as(
            "SELECT id, password FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }

    /// Find user by ID (aggregates profile and metadata)
    pub async fn find_user_by_id(&self, id: i32) -> Result<User, DatabaseError> {
        // 1. Fetch core info and profile
        let user_row = sqlx::query(
            "SELECT u.id, u.username, u.password, u.created_at, u.updated_at,
                    p.first_name AS prof_first_name, 
                    p.last_name AS prof_last_name, 
                    p.email AS prof_email
             FROM users u
             LEFT JOIN user_profiles p ON u.id = p.user_id
             WHERE u.id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        // 2. Fetch metadata
        let metadata_rows = sqlx::query(
            "SELECT parent_property, property, value FROM user_metadata WHERE user_id = ?"
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await?;

        let metadata = metadata_rows.into_iter().map(|row| {
            UserMetadata {
                parent_property: row.get(0),
                property: row.get(1),
                value: row.get(2),
            }
        }).collect();

        // Map optional profile fields
        let profile = if user_row.try_get::<String, _>("prof_first_name").is_ok() ||
                         user_row.try_get::<String, _>("prof_last_name").is_ok() ||
                         user_row.try_get::<String, _>("prof_email").is_ok() {
            Some(UserProfile {
                first_name: user_row.try_get("prof_first_name").ok(),
                last_name: user_row.try_get("prof_last_name").ok(),
                email: user_row.try_get("prof_email").ok(),
            })
        } else {
            None
        };

        Ok(User {
            id: user_row.get("id"),
            username: user_row.get("username"),
            password: user_row.get("password"),
            created_at: user_row.get("created_at"),
            updated_at: user_row.get("updated_at"),
            profile,
            metadata,
        })
    }
}
