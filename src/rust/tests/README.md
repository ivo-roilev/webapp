# Rust Backend Tests

This directory contains comprehensive test suites for the Rust user service backend.

## Test Organization

### Unit Tests
- **main_test.rs** - Unit tests for core functionality
- **user_info_formatter_test.rs** - Tests for user greeting formatter logic

### Integration Tests
- **handler_tests.rs** - HTTP handler integration tests with in-memory SQLite and mock logger

## Running Tests

### All Tests (Unit + Integration)
```bash
cargo test
```

### Unit Tests Only
```bash
cargo test main_test
cargo test user_info_formatter_test
```

### Integration Tests Only
```bash
cargo test handler_tests
```

### Sequential Execution (Recommended)
When running integration tests, use sequential execution to avoid test interference:
```bash
cargo test handler_tests -- --test-threads=1
```

### Single Test
```bash
cargo test handler_tests::test_create_user_success
```

## Handler Tests Architecture

The integration tests in `handler_tests.rs` use:
- **SQLite in-memory database** (`sqlite::memory:`) for fast, isolated testing without external MySQL
- **wiremock** for mocking the external logger service HTTP calls
- **actix-web test harness** for testing HTTP handlers end-to-end

### Test Infrastructure
- `setup_test_deps()` - Creates test database and mock logger
- `create_test_app()` - Builds actix-web App with routes
- `create_test_user()` - Helper to insert test users
- `assert_error_response()` - Validates error JSON structure

### Performance
Handler tests execute in **~0.36s** for 23 tests (sequential), compared to 5-10s with external services.

## Test Coverage

### Create User Endpoint (10 tests)
- Success scenarios (basic + optional fields)
- Validation errors (username/password length, empty fields)
- Duplicate username handling
- Logger integration and failure resilience

### Login Endpoint (5 tests)
- Successful authentication
- Invalid credentials (wrong password, non-existent user)
- Logging verification

### Get User Info Endpoint (6 tests)
- Success with valid ID
- Not found scenarios
- Invalid ID formats (non-numeric, negative)
- Logging verification

### Cross-Cutting Concerns (2 tests)
- Database isolation between test instances
- UNIQUE constraint enforcement

## Database Testing Strategy

**Production**: Uses `MySqlPool` connecting to external MySQL server
**Tests**: Uses `SqlitePool` with in-memory database via conditional compilation

The `Database` struct uses conditional compilation to switch pool types:
```rust
#[cfg(not(test))]
use sqlx::mysql::{MySqlPool as Pool, MySqlPoolOptions as PoolOptions};

#[cfg(test)]
use sqlx::sqlite::{SqlitePool as Pool, SqlitePoolOptions as PoolOptions};
```

This ensures:
- No external dependencies during test execution
- Parallel test execution with isolated databases
- Consistent test behavior across environments

## Troubleshooting

### Parallel Test Failures
Some tests may fail when run in parallel due to shared mock logger state. Use `--test-threads=1`:
```bash
cargo test handler_tests -- --test-threads=1
```

### Production Build
The SQLite test database code is excluded from production builds via `#[cfg(test)]`:
```bash
cargo build --release  # Only includes MySQL production code
```
