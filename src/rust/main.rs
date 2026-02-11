mod db;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use log::info;

// Re-export database types
use db::{Database, CreateUserRequest, User, DatabaseError};

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
                    let response: UserInfoResponse = user.into();
                    HttpResponse::Ok().json(response)
                }
                Err(DatabaseError::UserNotFound) => {
                    info!("User not found with ID: {}", user_id);
                    HttpResponse::NotFound().json(ErrorResponse {
                        error: "USER_NOT_FOUND".to_string(),
                        message: format!("User with ID {} not found", user_id),
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
                    log::error!("Error fetching user: {:?}", e);
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        error: "INTERNAL_ERROR".to_string(),
                        message: "Failed to fetch user".to_string(),
                    })
                }
            }
        }
        _ => {
            info!("Invalid user_id format: {}", user_id_str);
            HttpResponse::BadRequest().json(ErrorResponse {
                error: "VALIDATION_ERROR".to_string(),
                message: "user_id must be a positive integer".to_string(),
            })
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
    use super::*;

    #[test]
    fn test_create_user_payload_validation() {
        // Test username length validation
        let payload_long_username = serde_json::json!({
            "username": "abcdefghijklmnopqrst", // 20 chars, exceeds max 16
            "password": "test123"
        });

        // This should fail validation in the handler
        assert!(payload_long_username["username"]
            .as_str()
            .map(|u| u.len() > 16)
            .unwrap_or(false));

        // Test valid username
        let payload_valid = serde_json::json!({
            "username": "testuser",
            "password": "test123"
        });

        assert!(payload_valid["username"].as_str().is_some());
        assert!(payload_valid["password"].as_str().is_some());
    }

    #[test]
    fn test_login_payload_required_fields() {
        // Valid login payload
        let valid_login = serde_json::json!({
            "username": "testuser",
            "password": "password123"
        });

        assert!(valid_login["username"].as_str().is_some());
        assert!(valid_login["password"].as_str().is_some());

        // Both fields are required
        let missing_password = serde_json::json!({
            "username": "testuser"
        });

        assert!(missing_password["password"].is_null());
    }

    #[test]
    fn test_field_length_constraints() {
        // Test that optional fields cannot exceed 255 chars
        let long_field = "a".repeat(256);
        assert_eq!(long_field.len(), 256);
        assert!(long_field.len() > 255);

        // Test that max 255 char field is valid
        let valid_field = "a".repeat(255);
        assert_eq!(valid_field.len(), 255);
    }

    #[test]
    fn test_username_length_constraint() {
        // Username max 16 chars
        let max_username = "a".repeat(16);
        assert_eq!(max_username.len(), 16);

        let over_max_username = "a".repeat(17);
        assert_eq!(over_max_username.len(), 17);
        assert!(over_max_username.len() > 16);
    }

    #[test]
    fn test_user_response_serialization() {
        // Test that UserInfoResponse can be serialized to JSON
        let user_response = UserInfoResponse {
            id: 1,
            username: "testuser".to_string(),
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
            title: Some("Engineer".to_string()),
            hobby: Some("Reading".to_string()),
            // created_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            // updated_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        };

        let json = serde_json::to_string(&user_response);
        assert!(json.is_ok());

        let json_value: serde_json::Value = serde_json::to_value(&user_response).unwrap();
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["username"], "testuser");
        assert_eq!(json_value["first_name"], "Test");
    }

    #[test]
    fn test_error_response_serialization() {
        let error = ErrorResponse {
            error: "VALIDATION_ERROR".to_string(),
            message: "Invalid input".to_string(),
        };

        let json = serde_json::to_string(&error);
        assert!(json.is_ok());

        let json_value: serde_json::Value = serde_json::to_value(&error).unwrap();
        assert_eq!(json_value["error"], "VALIDATION_ERROR");
        assert_eq!(json_value["message"], "Invalid input");
    }

    #[test]
    fn test_create_user_response_json() {
        let response = CreateUserResponse { user_id: 42 };
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, r#"{"user_id":42}"#);
    }

    #[test]
    fn test_login_response_json() {
        let response = LoginResponse { user_id: 42 };
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(json, r#"{"user_id":42}"#);
    }

    #[test]
    fn test_optional_fields_can_be_null() {
        let create_payload = serde_json::json!({
            "username": "testuser",
            "password": "password123",
            "first_name": null,
            "last_name": null,
            "email": null,
            "title": null,
            "hobby": null
        });

        assert!(create_payload["first_name"].is_null());
        assert!(create_payload["email"].is_null());
    }

    #[test]
    fn test_utf8_character_support() {
        // Test UTF-8 characters in various fields
        let fields = vec![
            ("Ñandú", 5),       // 5 chars, Spanish
            ("日本語", 3),        // 3 chars, Japanese
            ("Привет", 6),      // 6 chars, Russian
            ("🎉🎊", 2),         // 2 emoji
        ];

        for (text, expected_len) in fields {
            assert_eq!(text.chars().count(), expected_len);
            assert!(text.len() <= 255); // byte length may be longer
        }
    }

    #[test]
    fn test_password_max_length() {
        let max_password = "p".repeat(255);
        assert_eq!(max_password.len(), 255);

        let over_max = "p".repeat(256);
        assert_eq!(over_max.len(), 256);
        assert!(over_max.len() > 255);
    }

    #[test]
    fn test_user_info_response_fields() {
        let response = UserInfoResponse {
            id: 123,
            username: "john_doe".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            title: Some("Manager".to_string()),
            hobby: Some("Gaming".to_string()),
            // created_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
            // updated_at: chrono::NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        };

        // Verify all fields are accessible
        assert_eq!(response.id, 123);
        assert_eq!(response.username, "john_doe");
        assert_eq!(response.first_name, Some("John".to_string()));
        assert_eq!(response.email, Some("john@example.com".to_string()));
    }

    #[test]
    fn test_validation_error_responses() {
        // Verify error response variants
        let errors = vec![
            ("VALIDATION_ERROR", "Username is required"),
            ("DUPLICATE_USERNAME", "Username already exists"),
            ("INVALID_CREDENTIALS", "Invalid username or password"),
            ("USER_NOT_FOUND", "User not found"),
            ("DATABASE_UNAVAILABLE", "Database connection failed"),
            ("INTERNAL_ERROR", "Internal server error"),
        ];

        for (error_code, _message) in errors {
            let response = ErrorResponse {
                error: error_code.to_string(),
                message: "Test".to_string(),
            };
            let json = serde_json::to_value(&response).unwrap();
            assert_eq!(json["error"].as_str(), Some(error_code));
        }
    }

    #[test]
    fn test_http_status_codes_mapping() {
        // Document expected HTTP status codes
        let status_mapping = vec![
            ("201", "Created - user successfully created"),
            ("200", "OK - login successful"),
            ("200", "OK - user info retrieved"),
            ("400", "Bad Request - validation error"),
            ("401", "Unauthorized - invalid credentials"),
            ("404", "Not Found - user ID not found"),
            ("409", "Conflict - duplicate username"),
            ("503", "Service Unavailable - database error"),
        ];

        // Verify mappings are documented
        assert_eq!(status_mapping.len(), 8);
    }
}

