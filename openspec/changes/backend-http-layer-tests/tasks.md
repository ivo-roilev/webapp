## 1. Dependencies & Configuration

- [x] 1.1 Add `sqlite` feature to `sqlx` dependency in Cargo.toml
- [x] 1.2 Add `wiremock = "0.6"` as dev-dependency in Cargo.toml
- [x] 1.3 Verify cargo check passes with new dependencies

## 2. Database Implementation - AnyPool Support

- [x] 2.1 Change Database struct pool field from `MySqlPool` to `sqlx::AnyPool` in src/rust/db.rs
- [x] 2.2 Update Database::new() to use AnyPoolOptions instead of MySqlPoolOptions
- [x] 2.3 Verify production MySQL connection still works with AnyPool
- [x] 2.4 Create SQLite-compatible schema SQL constant (AUTOINCREMENT, INTEGER types)
- [x] 2.5 Implement Database::new_test() constructor with #[cfg(test)] attribute
- [x] 2.6 Initialize in-memory SQLite pool in new_test() with "sqlite::memory:" URL
- [x] 2.7 Execute CREATE TABLE statement in new_test() to set up test schema
- [x] 2.8 Verify UNIQUE constraint on username works in SQLite test database
- [x] 2.9 Test Database::new_test() creates isolated instances for parallel tests

## 3. Test Infrastructure

- [x] 3.1 Create new file src/rust/tests/handler_tests.rs for integration tests
- [x] 3.2 Add mod handler_tests declaration in src/rust/tests/mod.rs or main.rs
- [x] 3.3 Implement setup_test_app() helper function that returns (TestApp, Database, MockServer)
- [x] 3.4 Configure LOGGER_URL environment variable in setup to point to wiremock
- [x] 3.5 Initialize actix-web test service with create_user, login, get_user_info routes
- [x] 3.6 Implement create_test_user() helper that directly inserts user into test DB
- [x] 3.7 Implement assert_error_response() helper for validating error JSON structure
- [x] 3.8 Verify test helpers compile and basic test harness works

## 4. Handler Integration Tests - Create User

- [x] 4.1 Test: Successful user creation returns 201 with user_id
- [x] 4.2 Test: User creation with optional fields stores all data
- [x] 4.3 Test: Username too long (>16 chars) returns 400 VALIDATION_ERROR
- [x] 4.4 Test: Empty username returns 400 VALIDATION_ERROR
- [x] 4.5 Test: Password too long (>255 chars) returns 400 VALIDATION_ERROR
- [x] 4.6 Test: Empty password returns 400 VALIDATION_ERROR
- [x] 4.7 Test: Optional field too long returns 400 VALIDATION_ERROR
- [x] 4.8 Test: Duplicate username returns 409 DUPLICATE_USERNAME
- [x] 4.9 Test: Verify create_user logs to mock logger service
- [x] 4.10 Test: Logger failure (500 error) does not break user creation

## 5. Handler Integration Tests - Login

- [x] 5.1 Test: Successful login with valid credentials returns 200 with user_id
- [x] 5.2 Test: Login with incorrect password returns 401 INVALID_CREDENTIALS
- [x] 5.3 Test: Login with non-existent username returns 401 INVALID_CREDENTIALS
- [x] 5.4 Test: Verify login logs "Login attempt" and "Successful login" events
- [x] 5.5 Test: Verify login log payload includes username in user field

## 6. Handler Integration Tests - Get User Info

- [x] 6.1 Test: Get user info with valid ID returns 200 with full user data
- [x] 6.2 Test: Get user info with non-existent ID returns 404 USER_NOT_FOUND
- [x] 6.3 Test: Get user info with invalid ID format (non-numeric) returns 400
- [x] 6.4 Test: Get user info with negative ID returns 400
- [x] 6.5 Test: Verify get_user_info logs "Fetching user info" event
- [x] 6.6 Test: Verify retrieved data includes all optional fields (first_name, email, etc.)

## 7. Logger Verification Tests

- [x] 7.1 Test: Verify log payload structure (timestamp, level, app, message fields)
- [x] 7.2 Test: Verify log level is "info" for successful operations
- [x] 7.3 Test: Verify log level is "error" for failed operations
- [x] 7.4 Test: Verify app field matches operation name (create_user, login_user, etc.)
- [x] 7.5 Test: Verify user field is populated when user context available
- [x] 7.6 Test: Mock logger can simulate failures without breaking handlers
- [x] 7.7 Test: Verify exact log call count using wiremock expectations

## 8. Verification & Polish

- [x] 8.1 Run cargo test and verify all 23 existing unit tests still pass
- [x] 8.2 Run cargo test handler_tests and verify all integration tests pass
- [x] 8.3 Verify cargo test completes in under 2 seconds total
- [x] 8.4 Test selective execution: cargo test main_test (unit tests only)
- [x] 8.5 Test selective execution: cargo test handler_tests (integration tests only)
- [x] 8.6 Verify production build (cargo build --release) still works
- [x] 8.7 Run existing MySQL database tests (src/database/ scripts) to ensure no regression
- [x] 8.8 Document test execution in src/rust/tests/README.md (if exists) or add comments
- [x] 8.9 Verify no production code changes except Database::pool type and new_test()
- [x] 8.10 Run openspec validate to ensure all artifacts are valid
