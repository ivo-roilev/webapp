mod db;
mod user_info_formatter;
mod logger;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files as fs;
use serde::{Deserialize, Serialize};
use crate::user_info_formatter::format_user_greeting;

// Re-export database types
use db::{Database, CreateUserRequest, User, DatabaseError, UserProfile, UserMetadata};

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
    pub extra_metadata: Option<Vec<UserMetadata>>,
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
    pub metadata: Vec<UserMetadata>,
}

impl From<User> for UserInfoResponse {
    fn from(user: User) -> Self {
        let (first_name, last_name, email) = user.profile.as_ref().map(|p| (p.first_name.clone(), p.last_name.clone(), p.email.clone())).unwrap_or((None, None, None));

        let mut title = None;
        let mut hobby = None;

        for m in &user.metadata {
            if m.property == "title" {
                title = m.value.clone();
            } else if m.property == "hobby" {
                hobby = m.value.clone();
            }
        }

        UserInfoResponse {
            id: user.id,
            username: user.username,
            first_name,
            last_name,
            email,
            title,
            hobby,
            metadata: user.metadata,
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
    payload: web::Form<CreateUserPayload>,
) -> impl Responder {
    log_info!(state.http_client, "create_user", payload.username, "Creating new user");

    // Validate required fields
    if payload.username.is_empty() || payload.username.len() > 16 {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/create-user.html?error=validation_error"))
            .finish();
    }

    if payload.password.is_empty() || payload.password.len() > 255 {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/create-user.html?error=validation_error"))
            .finish();
    }

    // Validate optional fields length
    if let Some(ref first_name) = payload.first_name {
        if first_name.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref last_name) = payload.last_name {
        if last_name.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref email) = payload.email {
        if email.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref title) = payload.title {
        if title.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref hobby) = payload.hobby {
        if hobby.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    let mut metadata = Vec::new();
    if let Some(ref title) = payload.title {
        metadata.push(UserMetadata {
            parent_property: None,
            property: "title".to_string(),
            value: Some(title.clone()),
        });
    }
    if let Some(ref hobby) = payload.hobby {
        metadata.push(UserMetadata {
            parent_property: None,
            property: "hobby".to_string(),
            value: Some(hobby.clone()),
        });
    }
    if let Some(ref extra) = payload.extra_metadata {
        metadata.extend(extra.clone());
    }

    let create_request = CreateUserRequest {
        username: payload.username.clone(),
        password: payload.password.clone(),
        profile: Some(UserProfile {
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            email: payload.email.clone(),
        }),
        metadata,
    };

    match state.db.create_user(&create_request).await {
        Ok(user_id) => {
            log_info!(state.http_client, "create_user", payload.username, "User created successfully with ID: {}", user_id);
            HttpResponse::SeeOther()
                .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                .finish()
        }
        Err(DatabaseError::DuplicateUsername) => {
            log_info!(state.http_client, "create_user", payload.username, "Username already exists");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=duplicate_username"))
                .finish()
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "create_user", payload.username, "Database connection error");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=database_error"))
                .finish()
        }
        Err(e) => {
            log_error!(state.http_client, "create_user", payload.username, "Error creating user: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=server_error"))
                .finish()
        }
    }
}

/// POST /api/login - Login with username and password
async fn login(
    state: web::Data<AppState>,
    payload: web::Form<LoginPayload>,
) -> impl Responder {
    log_info!(state.http_client, "login_user", payload.username, "Login attempt");

    // Validate required fields
    if payload.username.is_empty() {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/index.html?error=validation_error"))
            .finish();
    }

    if payload.password.is_empty() {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/index.html?error=validation_error"))
            .finish();
    }

    match state.db.authenticate_user(&payload.username).await {
        Ok((user_id, stored_password)) => {
            // Compare passwords (plain-text comparison as per design)
            if stored_password == payload.password {
                log_info!(state.http_client, "login_user", payload.username, "Successful login");
                HttpResponse::SeeOther()
                    .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                    .finish()
            } else {
                log_info!(state.http_client, "login_user", payload.username, "Invalid password");
                HttpResponse::SeeOther()
                    .append_header(("Location", "/index.html?error=invalid_credentials"))
                    .finish()
            }
        }
        Err(DatabaseError::UserNotFound) => {
            log_info!(state.http_client, "login_user", payload.username, "User not found during login");
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=invalid_credentials"))
                .finish()
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "login_user", payload.username, "Database connection error");
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=database_error"))
                .finish()
        }
        Err(e) => {
            log_error!(state.http_client, "login_user", payload.username, "Error during login: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=server_error"))
                .finish()
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
                    let greeting = format_user_greeting(&user);
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
            .route("/api/create-user", web::post().to(create_user))
            .route("/api/login", web::post().to(login))
            .route("/api/users/{user_id}", web::get().to(get_user_info))
            .service(
                fs::Files::new("/", "./src/web")
                    .index_file("index.html")
                    .use_last_modified(true)
            )
    })
    .bind(&bind_addr)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    mod main_test;
    mod user_info_formatter_test;
    mod handler_tests;
}

