## Context

Current test suite ([src/rust/tests/main_test.rs](src/rust/tests/main_test.rs)) covers serialization, validation logic, and data structure tests but does not test HTTP handlers. To test handlers end-to-end today requires:
- MySQL running on localhost:3306 with schema initialized
- Logger service running if LOGGER_URL is configured
- Manual cleanup between tests to avoid data pollution

This creates a slow, brittle test environment (~5-10s startup overhead) and prevents testing error scenarios like database connection failures or logger service outages.

**Existing test coverage:**
- MySQL schema tests exist at [src/database/](src/database/) including:
  - Schema validation ([01_users_schema.sql](src/database/01_users_schema.sql))
  - Functional test suite ([04_test_suite.sql](src/database/04_test_suite.sql)) - validates DDL operations, constraints, and MySQL-specific behavior
  - Security tests ([05_security_tests.sh](src/database/05_security_tests.sh)) - privilege boundary validation
- These tests validate MySQL schema correctness and database-level constraints
- Gap: No tests for HTTP handler logic and application-level error handling

The application uses:
- Actix Web 4.x for HTTP routing and handlers
- sqlx with MySqlPool for database connections
- reqwest::Client for HTTP calls to logger service
- Handlers tightly coupled to concrete Database type with MySqlPool

## Goals / Non-Goals

**Goals:**
- Enable fast HTTP handler tests (<1s total execution time)
- Test handlers without external MySQL or logger service dependencies
- Support testing error scenarios (connection failures, logger outages, duplicate keys, not found, etc.)
- Preserve production MySQL behavior - no changes to runtime code paths
- Keep existing unit tests unchanged - additive change only

**Non-Goals:**
- Replace production MySQL with SQLite
- Test MySQL-specific dialect features or connection pool behavior
- Implement a full test container solution (can be added later if needed)
- Refactor handlers to use dependency injection traits (avoid architecture churn)

## Decisions

### Decision 1: SQLite for test database (not MySQL testcontainers)

**Choice**: Use SQLite with in-memory mode for test database

**Rationale**:
- **Speed**: SQLite setup is <10ms vs 5-10s for MySQL container startup
- **Zero dependencies**: No Docker required, tests run in CI/local without setup
- **Sufficient coverage**: 95% of test scenarios don't require MySQL-specific features
- **Existing MySQL tests**: MySQL-specific behavior (schema, constraints, privileges) is already validated by [src/database/](src/database/) test suite - application logic tests don't need to duplicate this
- **Additive**: Can add testcontainers later for MySQL-specific integration tests if needed

**Alternatives considered**:
- MySQL testcontainers: Rejected due to slow startup and Docker dependency
- Mock trait for Database: Rejected to avoid refactoring handlers and adding trait complexity

**Trade-offs**:
- SQLite uses different SQL dialect (e.g., `AUTOINCREMENT` vs `AUTO_INCREMENT`)
- Cannot test MySQL-specific behaviors (connection pooling, charset issues, stored procedures)
- Schema must be maintained in test code (no shared migration files with production)

**Mitigation**: Document dialect differences in test code. If production bugs surface related to MySQL-specific features, add targeted testcontainer tests for those cases.

### Decision 2: Use sqlx::AnyPool for database abstraction

**Choice**: Change Database struct to use `sqlx::AnyPool` instead of `MySqlPool`

**Rationale**:
- AnyPool supports both MySQL and SQLite with a unified interface
- Minimal code changes - query syntax stays the same for 95% of cases
- Production code automatically works with test database without conditional compilation
- Single codebase for both production and test database implementations

**Alternatives considered**:
- Conditional compilation (`#[cfg(test)]` with separate SqlitePool): Rejected due to maintaining two codebases
- Separate TestDatabase struct: Rejected due to code duplication and drift risk
- MockDatabase trait: Rejected due to refactoring scope and handler signature changes

**Implementation**:
```rust
// In db.rs
pub struct Database {
    pool: sqlx::AnyPool,  // Was: MySqlPool
}

impl Database {
    // Production constructor (unchanged behavior)
    pub async fn new() -> Result<Self, DatabaseError> {
        let database_url = /* build MySQL URL */;
        let pool = sqlx::AnyPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await?;
        Ok(Database { pool })
    }

    // Test constructor (new)
    #[cfg(test)]
    pub async fn new_test() -> Result<Self, DatabaseError> {
        let pool = sqlx::AnyPoolOptions::new()
            .connect("sqlite::memory:")
            .await?;

        // Create schema inline (SQLite-compatible SQL)
        sqlx::query(CREATE_USERS_TABLE_SQLITE).execute(&pool).await?;

        Ok(Database { pool })
    }
}
```

**Trade-offs**:
- Slightly less type safety (AnyPool uses dynamic dispatch internally)
- Query methods must handle dialect differences explicitly for edge cases
- Small runtime overhead from dynamic dispatch (negligible in practice)

### Decision 3: wiremock for logger service mocking

**Choice**: Use wiremock crate to mock HTTP logger service endpoints

**Rationale**:
- Provides full HTTP mock server that matches logger service API contract
- Can verify exact payloads, headers, and call counts
- Supports simulating failures (500 errors, timeouts, network errors)
- Well-maintained crate with good ergonomics for actix-web tests

**Alternatives considered**:
- Skip logger testing (set LOGGER_URL=""):
Rejected because we want to verify logging behavior and resilience to logger failures
- Custom HTTP mock: Rejected due to reinventing the wheel

**Implementation pattern**:
```rust
#[actix_web::test]
async fn test_create_user_with_logging() {
    let mock_logger = MockServer::start().await;
    std::env::set_var("LOGGER_URL", format!("{}/logs", mock_logger.uri()));

    Mock::given(method("POST"))
        .and(path("/logs"))
        .and(body_json_string(|body| {
            let json: Value = serde_json::from_str(body)?;
            json["app"] == "create_user" && json["level"] == "info"
        }))
        .respond_with(ResponseTemplate::new(200))
        .expect(2)  // Verify called exactly twice
        .mount(&mock_logger)
        .await;

    // ... test handler ...
}
```

### Decision 4: Test structure - separate integration test module

**Choice**: Create new `src/rust/tests/handler_tests.rs` for HTTP layer tests

**Rationale**:
- Keeps unit tests ([main_test.rs](src/rust/tests/main_test.rs)) fast and focused
- Integration tests use `#[actix_web::test]` attribute which requires tokio runtime
- Clear separation: unit tests run first (fast feedback), integration tests run second
- Allows selective test execution: `cargo test main_test` vs `cargo test handler_tests`

**Test helper pattern**:
```rust
async fn setup_test_app() -> (TestApp, MockServer) {
    let db = Database::new_test().await.unwrap();
    let mock_logger = MockServer::start().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(AppState {
                db: Arc::new(db),
                http_client: Arc::new(reqwest::Client::new()),
            }))
            .route("/api/users", web::post().to(create_user))
            .route("/api/login", web::post().to(login))
            .route("/api/users/{id}", web::get().to(get_user_info))
    ).await;

    (app, mock_logger)
}
```

## Risks / Trade-offs

**Risk**: SQLite dialect differences cause tests to pass but production MySQL fails
→ **Mitigation**: Document known differences. Add testcontainer tests for MySQL-specific features if issues arise in production. Focus on business logic testing, not SQL edge cases.

**Risk**: AnyPool performance overhead or bugs
→ **Mitigation**: AnyPool is mature and well-tested. Performance impact is negligible (<1% overhead from dynamic dispatch). Can revert to MySqlPool if issues found.

**Risk**: Test schema drift from production schema
→ **Mitigation**: Keep test schema simple and focused on core fields. Document that tests don't validate schema migrations or complex constraints.

**Risk**: wiremock adds test complexity
→ **Mitigation**: Create reusable test helper for logger setup. Most tests can skip logger verification by not setting expectations.

**Trade-off**: Tests won't catch MySQL-specific issues
→ **Accept**: 95% of bugs are logic errors, not SQL dialect issues. Can add targeted MySQL tests later if needed.

## Migration Plan

Not applicable - this is additive change with no production code modifications beyond Database pool type change.

Deployment steps:
1. Update Cargo.toml dependencies (sqlx sqlite feature, wiremock dev-dependency)
2. Change Database::pool from MySqlPool to AnyPool
3. Add Database::new_test() constructor
4. Create handler_tests.rs with initial test coverage
5. Verify all existing tests still pass
6. Run new handler tests

Rollback: Revert Database::pool to MySqlPool if AnyPool causes issues (unlikely).

## Open Questions

None - all key decisions are resolved based on exploration discussion.
