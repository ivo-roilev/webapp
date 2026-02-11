use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use sqlx::Row;
use std::time::Duration;
// use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub hobby: Option<String>,
    // pub created_at: NaiveDateTime,
    // pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub hobby: Option<String>,
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

pub struct Database {
    pool: MySqlPool,
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

        let pool: MySqlPool = MySqlPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&database_url)
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Database { pool })
    }

    /// Create a new user record in the database
    pub async fn create_user(&self, user: &CreateUserRequest) -> Result<i32, DatabaseError> {
        let result = sqlx::query(
            "INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.title)
        .bind(&user.hobby)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_id() as i32)
    }

    /// Find user by username
    pub async fn find_user_by_username(&self, username: &str) -> Result<User, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
             FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_user_by_id(&self, id: i32) -> Result<User, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password, first_name, last_name, email, title, hobby
             FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

// Implement sqlx::FromRow for User struct
impl sqlx::FromRow<'_, sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            email: row.try_get("email")?,
            title: row.try_get("title")?,
            hobby: row.try_get("hobby")?,
            // created_at: row.try_get("created_at")?,
            // updated_at: row.try_get("updated_at")?,
        })
    }
}
