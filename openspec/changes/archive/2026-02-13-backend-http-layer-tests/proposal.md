## Why

Current tests only validate data structures and serialization logic, not the actual HTTP handler behavior. Testing handlers end-to-end requires MySQL database and logger service to be running, making tests slow, environment-dependent, and unable to simulate error scenarios like connection failures or logger service outages. This limits test coverage and developer velocity.

## What Changes

- Add HTTP layer integration tests using actix-web's test harness to test handlers without external services
- Integrate SQLite as an in-memory test database to eliminate MySQL dependency during testing
- Use wiremock to mock the remote logger service HTTP endpoints, enabling verification of logging behavior and simulation of logger failures
- Extend Database implementation to support both MySQL (production) and SQLite (testing) via sqlx::AnyPool
- Add new test module with comprehensive handler tests covering success paths, validation errors, database errors, and logging verification

## Capabilities

### New Capabilities
- `handler-integration-tests`: HTTP layer testing framework that validates endpoint handlers with mocked dependencies (database and logger service), supporting fast test execution and comprehensive error scenario coverage

### Modified Capabilities
- `mysql-database-integration`: Extend Database struct to support both MySQL and SQLite connection pools, enabling production MySQL usage and in-memory SQLite for testing

## Impact

**Dependencies:**
- Add `wiremock` crate (dev-dependency) for HTTP mocking
- Add `sqlite` feature to existing `sqlx` dependency
- No production dependency changes

**Code:**
- `src/rust/db.rs`: Database struct pool field changes from `MySqlPool` to `AnyPool`, add `new_test()` constructor
- `src/rust/tests/`: New `handler_tests.rs` module with integration tests
- `Cargo.toml`: Updated dependencies

**Testing:**
- Existing unit tests remain unchanged
- New integration tests run in ~0.4s vs ~5-10s for external service tests
- Tests can now simulate database errors, logger failures, and network issues
