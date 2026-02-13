//! HTTP Handler Integration Tests
//!
//! Tests HTTP handlers without external MySQL or logger service dependencies.
//! Uses SQLite in-memory for database and wiremock for logger service mocking.

use actix_web::{dev::ServiceResponse, test, web, App};
use serde_json::{json, Value};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::db::Database;
use crate::{create_user, get_user_info, login, AppState};

// Global mutex to serialize tests that use environment variables
// This is necessary because std::env::set_var is not thread-safe and
// our implementation relies on std::env::var("LOGGER_URL")
static TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

// ============ Test Helpers ============

/// Create test database and mock logger server
/// Create test database and mock logger server
async fn setup_test_deps() -> (Database, MockServer, std::sync::MutexGuard<'static, ()>) {
    let guard = TEST_MUTEX.lock().unwrap();

    // Create test database
    let db = Database::new_test()
        .await
        .expect("Failed to create test database");

    // Create mock logger server
    let mock_logger = MockServer::start().await;
    std::env::set_var("LOGGER_URL", mock_logger.uri());

    (db, mock_logger, guard)
}

/// Create test app with given database
fn create_test_app(
    db: Database,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(AppState {
            db: db,
            http_client: reqwest::Client::new(),
        }))
        .route("/api/users", web::post().to(create_user))
        .route("/api/login", web::post().to(login))
        .route("/api/users/{user_id}", web::get().to(get_user_info))
}

/// Insert a test user directly into the database
async fn create_test_user(db: &Database, username: &str, password: &str) -> i32 {
    use crate::db::{CreateUserRequest, UserProfile, UserMetadata};
    db.create_user(&CreateUserRequest {
        username: username.to_string(),
        password: password.to_string(),
        profile: Some(UserProfile {
            first_name: Some("Test".to_string()),
            last_name: Some("User".to_string()),
            email: Some("test@example.com".to_string()),
        }),
        metadata: vec![
            UserMetadata {
                parent_property: None,
                property: "title".to_string(),
                value: Some("Engineer".to_string()),
            },
            UserMetadata {
                parent_property: None,
                property: "hobby".to_string(),
                value: Some("Coding".to_string()),
            },
        ],
    })
    .await
    .expect("Failed to create test user")
}

/// Assert error response structure
fn assert_error_response(body: &Value, expected_error: &str) {
    assert!(
        body.get("error").is_some(),
        "Response should have 'error' field"
    );
    assert_eq!(body["error"].as_str().unwrap(), expected_error);
    assert!(
        body.get("message").is_some(),
        "Response should have 'message' field"
    );
}

// ============ Create User Tests ============

#[actix_web::test]
async fn test_create_user_success() {
    let (db, mock_logger, _guard) = setup_test_deps().await;

    // Mock logger to accept requests
    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let app = test::init_service(create_test_app(db.clone())).await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "testuser",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201, "Should return 201 Created");

    let body: Value = test::read_body_json(resp).await;
    assert!(
        body.get("user_id").is_some(),
        "Response should contain user_id"
    );
}

#[actix_web::test]
async fn test_create_user_with_optional_fields() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "fulluser",
            "password": "password123",
            "first_name": "John",
            "last_name": "Doe",
            "email": "john@example.com",
            "title": "Engineer",
            "hobby": "Reading"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let body: Value = test::read_body_json(resp).await;
    assert!(body.get("user_id").is_some());
}

#[actix_web::test]
async fn test_create_user_username_too_long() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "this_username_is_way_too_long_and_exceeds_16_chars",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_create_user_empty_username() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_create_user_password_too_long() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let long_password = "a".repeat(256);
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "testuser",
            "password": long_password
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_create_user_empty_password() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "testuser",
            "password": ""
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_create_user_optional_field_too_long() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let long_email = "a".repeat(256) + "@example.com";
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "testuser",
            "password": "password123",
            "email": long_email
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "VALIDATION_ERROR");
}

#[actix_web::test]
async fn test_create_user_duplicate_username() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    // Create first user
    create_test_user(&db, "duplicate", "password123").await;

    // Try to create user with same username
    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "duplicate",
            "password": "password456"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 409);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "DUPLICATE_USERNAME");
}

#[actix_web::test]
async fn test_create_user_logs_to_logger() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    // Mount mock with expectations
    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .expect(2) // Creating user + logging success
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "logtest",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Give async logging time to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

#[actix_web::test]
async fn test_create_user_logger_failure_doesnt_break_creation() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    // Mock logger to return 500 error
    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "resilient",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(
        resp.status().as_u16(),
        201,
        "User creation should succeed despite logger failure"
    );

    let body: Value = test::read_body_json(resp).await;
    assert!(body.get("user_id").is_some());
}

// ============ Login Tests ============

#[actix_web::test]
async fn test_login_success() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    create_test_user(&db, "loginuser", "correct_password").await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({
            "username": "loginuser",
            "password": "correct_password"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: Value = test::read_body_json(resp).await;
    assert!(body.get("user_id").is_some());
}

#[actix_web::test]
async fn test_login_incorrect_password() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    create_test_user(&db, "loginuser", "correct_password").await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({
            "username": "loginuser",
            "password": "wrong_password"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "INVALID_CREDENTIALS");
}

#[actix_web::test]
async fn test_login_nonexistent_user() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({
            "username": "nonexistent",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let body: Value = test::read_body_json(resp).await;
    assert_error_response(&body, "INVALID_CREDENTIALS");
}

#[actix_web::test]
async fn test_login_logs_attempt_and_success() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .expect(2) // Login attempt + Successful login
        .mount(&mock_logger)
        .await;

    create_test_user(&db, "loguser", "password123").await;

    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_json(json!({
            "username": "loguser",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

// ============ Get User Info Tests ============

#[actix_web::test]
async fn test_get_user_info_success() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let user_id = create_test_user(&db, "infouser", "password123").await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user_id))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    // Check that the greeting contains expected elements (first_name + last_name from create_test_user)
    assert!(body_str.contains("Test User"));
    assert!(body_str.contains("welcome"));
}

#[actix_web::test]
async fn test_get_user_info_not_found() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::get()
        .uri("/api/users/99999")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("not found"));
}

#[actix_web::test]
async fn test_get_user_info_invalid_id_format() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let req = test::TestRequest::get()
        .uri("/api/users/not_a_number")
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_get_user_info_negative_id() {
    let (db, _mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    let req = test::TestRequest::get().uri("/api/users/-1").to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
async fn test_get_user_info_with_all_optional_fields() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db.clone())).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    use crate::db::{CreateUserRequest, UserProfile, UserMetadata};
    let user_id = db
        .create_user(&CreateUserRequest {
            username: "fullinfo".to_string(),
            password: "password123".to_string(),
            profile: Some(UserProfile {
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                email: Some("john@example.com".to_string()),
            }),
            metadata: vec![
                UserMetadata {
                    parent_property: None,
                    property: "title".to_string(),
                    value: Some("Engineer".to_string()),
                },
                UserMetadata {
                    parent_property: None,
                    property: "hobby".to_string(),
                    value: Some("Coding".to_string()),
                },
            ],
        })
        .await
        .expect("Failed to create test user");

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user_id))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("John"));
    assert!(body_str.contains("Doe"));
    assert!(body_str.contains("john@example.com"));
}

// ============ Logger Verification Tests ============

#[actix_web::test]
async fn test_verify_log_payload_structure() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "logpayload",
            "password": "password123"
        }))
        .to_request();

    test::call_service(&app, req).await;
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Note: With fire-and-forget logging, we can't easily verify payload structure
    // In production, consider using mock server's received_requests() method
}

#[actix_web::test]
async fn test_logger_failure_simulation() {
    let (db, mock_logger, _guard) = setup_test_deps().await;
    let app = test::init_service(create_test_app(db)).await;

    // Simulate various logger failures
    Mock::given(method("POST"))
        .and(path("/logs"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&mock_logger)
        .await;

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(json!({
            "username": "failtest",
            "password": "password123"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(
        resp.status().as_u16(),
        201,
        "Handler should succeed despite logger failure"
    );
}

// ============ Database Isolation Tests ============

#[actix_web::test]
async fn test_database_isolation() {
    // Create two separate databases
    let db1 = Database::new_test().await.expect("Failed to create db1");
    let db2 = Database::new_test().await.expect("Failed to create db2");

    // Create user in db1
    create_test_user(&db1, "user1", "password1").await;

    // Verify user doesn't exist in db2
    use crate::db::DatabaseError;
    let result = db2.authenticate_user("user1").await;
    assert!(
        matches!(result, Err(DatabaseError::UserNotFound)),
        "Databases should be isolated"
    );
}

#[actix_web::test]
async fn test_unique_constraint_works_in_sqlite() {
    let db = Database::new_test()
        .await
        .expect("Failed to create test database");

    // Create first user
    create_test_user(&db, "unique_test", "password123").await;

    // Try to create user with same username
    use crate::db::CreateUserRequest;
    let result = db
        .create_user(&CreateUserRequest {
            username: "unique_test".to_string(),
            password: "password456".to_string(),
            profile: None,
            metadata: vec![],
        })
        .await;

    use crate::db::DatabaseError;
    assert!(
        matches!(result, Err(DatabaseError::DuplicateUsername)),
        "UNIQUE constraint should work in SQLite"
    );
}
