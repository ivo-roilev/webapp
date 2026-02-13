mod db;
mod user_info_formatter;
mod logger;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

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

// ============ Application State ============

struct AppState {
    db: Database,
    http_client: reqwest::Client,
}

// ============ Endpoint Handlers ============

/// POST /api/users - Create a new user
async fn create_user(
    state: web::Data<AppState>,
    payload: web::Json<CreateUserPayload>,
) -> impl Responder {
    log_info!(state.http_client, "create_user", payload.username, "Creating new user");

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

    match state.db.create_user(&create_request).await {
        Ok(user_id) => {
            log_info!(state.http_client, "create_user", payload.username, "User created successfully with ID: {}", user_id);
            HttpResponse::Created().json(CreateUserResponse { user_id })
        }
        Err(DatabaseError::DuplicateUsername) => {
            log_info!(state.http_client, "create_user", payload.username, "Username already exists");
            HttpResponse::Conflict().json(ErrorResponse {
                error: "DUPLICATE_USERNAME".to_string(),
                message: format!("Username '{}' already exists", payload.username),
            })
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "create_user", payload.username, "Database connection error");
            HttpResponse::ServiceUnavailable().json(ErrorResponse {
                error: "DATABASE_UNAVAILABLE".to_string(),
                message: "Database connection failed".to_string(),
            })
        }
        Err(e) => {
            log_error!(state.http_client, "create_user", payload.username, "Error creating user: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "Failed to create user".to_string(),
            })
        }
    }
}

/// POST /api/login - Login with username and password
async fn login(
    state: web::Data<AppState>,
    payload: web::Json<LoginPayload>,
) -> impl Responder {
    log_info!(state.http_client, "login_user", payload.username, "Login attempt");

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

    match state.db.find_user_by_username(&payload.username).await {
        Ok(user) => {
            // Compare passwords (plain-text comparison as per design)
            if user.password == payload.password {
                log_info!(state.http_client, "login_user", payload.username, "Successful login");
                HttpResponse::Ok().json(LoginResponse { user_id: user.id })
            } else {
                log_info!(state.http_client, "login_user", payload.username, "Invalid password");
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        Err(DatabaseError::UserNotFound) => {
            log_info!(state.http_client, "login_user", payload.username, "User not found during login");
            HttpResponse::Unauthorized().json(ErrorResponse {
                error: "INVALID_CREDENTIALS".to_string(),
                message: "Invalid username or password".to_string(),
            })
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "login_user", payload.username, "Database connection error");
            HttpResponse::ServiceUnavailable().json(ErrorResponse {
                error: "DATABASE_UNAVAILABLE".to_string(),
                message: "Database connection failed".to_string(),
            })
        }
        Err(e) => {
            log_error!(state.http_client, "login_user", payload.username, "Error during login: {:?}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "INTERNAL_ERROR".to_string(),
                message: "Login failed".to_string(),
            })
        }
    }
}

/// GET /api/users/{user_id} - Get user information
async fn get_user_info(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id_str = path.into_inner();

    // Validate user_id format and parse
    match user_id_str.parse::<i32>() {
        Ok(user_id) if user_id > 0 => {
            log_info!(state.http_client, "get_user_info", user_id, "Fetching user info");

            match state.db.find_user_by_id(user_id).await {
                Ok(user) => {
                    let username = user.username.clone();
                    log_info!(state.http_client, "get_user_info", username, "User info retrieved for ID: {}", user_id);
                    let greeting = format_user_greeting(user);
                    HttpResponse::Ok()
                        .content_type("text/plain; charset=utf-8")
                        .body(greeting)
                }
                Err(DatabaseError::UserNotFound) => {
                    log_info!(state.http_client, "get_user_info", user_id, "User not found");
                    HttpResponse::NotFound()
                        .content_type("text/plain; charset=utf-8")
                        .body(format!("User with ID {} not found", user_id))
                }
                Err(DatabaseError::ConnectionError(_)) => {
                    log_error!(state.http_client, "get_user_info", "", "Database connection error");
                    HttpResponse::ServiceUnavailable()
                        .content_type("text/plain; charset=utf-8")
                        .body("Database connection failed")
                }
                Err(e) => {
                    log_error!(state.http_client, "get_user_info", user_id, "Error fetching user: {:?}", e);
                    HttpResponse::InternalServerError()
                        .content_type("text/plain; charset=utf-8")
                        .body("Failed to fetch user")
                }
            }
        }
        Ok(_) => {
            // Negative or zero user_id
            log_info!(state.http_client, "get_user_info", user_id_str, "Invalid user_id (non-positive)");
            HttpResponse::BadRequest()
                .content_type("text/plain; charset=utf-8")
                .body("user_id must be a positive integer")
        }
        Err(_) => {
            // Non-numeric user_id
            log_info!(state.http_client, "get_user_info", user_id_str, "Invalid user_id format");
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

    // Create HTTP client for dual logging
    let http_client = reqwest::Client::new();

    // Initialize database connection pool
    let db = match Database::new().await {
        Ok(db) => db,
        Err(e) => {
            log_error!(http_client, "main", "SYSTEM", "Failed to initialize database: {:?}", e);
            panic!("Cannot start server: database initialization failed");
        }
    };

    let state = web::Data::new(AppState {
        db,
        http_client,
    });

    let server_host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", server_host, server_port);

    log_info!(state.http_client, "main", "SYSTEM", "Starting HTTP server on {}", bind_addr);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(state.clone())
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

