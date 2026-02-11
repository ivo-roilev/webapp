mod db;
mod user_info_formatter;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::info;

// Re-export database types
use db::{Database, CreateUserRequest, User, DatabaseError};
use user_info_formatter::format_user_greeting;

// ============ Request/Response Structs ============

#[derive(Debug, Deserialize)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub hobby: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub title: Option<String>,
    pub hobby: Option<String>,
//    pub created_at: chrono::NaiveDateTime,
    // pub updated_at: chrono::NaiveDateTime,
}

impl From<User> for UserInfoResponse {
    fn from(user: User) -> Self {
        UserInfoResponse {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            title: user.title,
            hobby: user.hobby,
            // created_at: user.created_at,
            // updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

// ============ Endpoint Handlers ============

/// POST /api/users - Create a new user
async fn create_user(
    db: web::Data<Arc<Database>>,
    payload: web::Json<CreateUserPayload>,
) -> impl Responder {
    info!("Creating new user: {}", payload.username);

    // Validate required fields
    if payload.username.is_empty() || payload.username.len() > 16 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Username is required and must be max 16 characters".to_string(),
        });
    }

    if payload.password.is_empty() || payload.password.len() > 255 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Password is required and must be max 255 characters".to_string(),
        });
    }

    // Validate optional fields length
    if let Some(ref first_name) = payload.first_name {
        if first_name.len() > 255 {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "first_name must be max 255 characters".to_string(),
            });
        }
    }

    if let Some(ref last_name) = payload.last_name {
        if last_name.len() > 255 {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "last_name must be max 255 characters".to_string(),
            });
        }
    }

    if let Some(ref email) = payload.email {
        if email.len() > 255 {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "email must be max 255 characters".to_string(),
            });
        }
    }

    if let Some(ref title) = payload.title {
        if title.len() > 255 {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "title must be max 255 characters".to_string(),
            });
        }
    }

    if let Some(ref hobby) = payload.hobby {
        if hobby.len() > 255 {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "hobby must be max 255 characters".to_string(),
            });
        }
    }

    let create_request = CreateUserRequest {
        username: payload.username.clone(),
        password: payload.password.clone(),
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        email: payload.email.clone(),
        title: payload.title.clone(),
        hobby: payload.hobby.clone(),
    };

    match db.create_user(&create_request).await {
        Ok(user_id) => {
            info!("User created successfully with ID: {}", user_id);
            HttpResponse::Created().json(CreateUserResponse { user_id })
        }
        Err(DatabaseError::DuplicateUsername) => {
            info!("Username {} already exists", payload.username);
            HttpResponse::Conflict().json(ErrorResponse {
                error: "DUPLICATE_USERNAME".to_string(),
                message: format!("Username '{}' already exists", payload.username),
            })
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log::error!("Database connection error");
            HttpResponse::ServiceUnavailable().json(ErrorResponse {
                error: "DATABASE_UNAVAILABLE".to_string(),
                message: "Database connection failed".to_string(),
            })
        }
        Err(e) => {
            log::error!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "Failed to create user".to_string(),
            })
        }
    }
}

/// POST /api/login - Login with username and password
async fn login(
    db: web::Data<Arc<Database>>,
    payload: web::Json<LoginPayload>,
) -> impl Responder {
    info!("Login attempt for user: {}", payload.username);

    // Validate required fields
    if payload.username.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Username is required".to_string(),
        });
    }

    if payload.password.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Password is required".to_string(),
        });
    }

    match db.find_user_by_username(&payload.username).await {
        Ok(user) => {
            // Compare passwords (plain-text comparison as per design)
            if user.password == payload.password {
                info!("Successful login for user: {}", payload.username);
                HttpResponse::Ok().json(LoginResponse { user_id: user.id })
            } else {
                info!("Invalid password for user: {}", payload.username);
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        Err(DatabaseError::UserNotFound) => {
            info!("User not found during login: {}", payload.username);
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username or password".to_string(),
            })
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log::error!("Database connection error");
            HttpResponse::ServiceUnavailable().json(ErrorResponse {
                error: "DATABASE_UNAVAILABLE".to_string(),
                message: "Database connection failed".to_string(),
            })
        }
        Err(e) => {
            log::error!("Error during login: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "Login failed".to_string(),
            })
        }
    }
}

/// GET /api/users/{user_id} - Get user information
async fn get_user_info(
    db: web::Data<Arc<Database>>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id_str = path.into_inner();

    // Validate user_id format and parse
    match user_id_str.parse::<i32>() {
        Ok(user_id) if user_id > 0 => {
            info!("Fetching user info for ID: {}", user_id);

            match db.find_user_by_id(user_id).await {
                Ok(user) => {
                    info!("User info retrieved for ID: {}", user_id);
                    let greeting = format_user_greeting(user);
                    HttpResponse::Ok()
                        .content_type("text/plain; charset=utf-8")
                        .body(greeting)
                }
                Err(DatabaseError::UserNotFound) => {
                    info!("User not found with ID: {}", user_id);
                    HttpResponse::NotFound()
                        .content_type("text/plain; charset=utf-8")
                        .body(format!("User with ID {} not found", user_id))
                }
                Err(DatabaseError::ConnectionError(_)) => {
                    log::error!("Database connection error");
                    HttpResponse::ServiceUnavailable()
                        .content_type("text/plain; charset=utf-8")
                        .body("Database connection failed")
                }
                Err(e) => {
                    log::error!("Error fetching user: {:?}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/plain; charset=utf-8")
                        .body("Failed to fetch user")
                }
            }
        }
        Ok(_) => {
            // Negative or zero user_id
            info!("Invalid user_id (non-positive): {}", user_id_str);
            HttpResponse::BadRequest()
                .content_type("text/plain; charset=utf-8")
                .body("user_id must be a positive integer")
        }
        Err(_) => {
            // Non-numeric user_id
            info!("Invalid user_id format: {}", user_id_str);
            HttpResponse::BadRequest()
                .content_type("text/plain; charset=utf-8")
                .body("user_id must be a valid integer")
        }
    }
}

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize database connection pool
    let db = match Database::new().await {
        Ok(db) => Arc::new(db),
        Err(e) => {
            log::error!("Failed to initialize database: {:?}", e);
            panic!("Cannot start server: database initialization failed");
        }
    };

    let db_data = web::Data::new(db);

    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", server_host, server_port);

    info!("Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/health", web::get().to(health_check))
            .route("/api/users", web::post().to(create_user))
            .route("/api/login", web::post().to(login))
            .route("/api/users/{user_id}", web::get().to(get_user_info))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    mod main_test;
    mod user_info_formatter_test;
}

