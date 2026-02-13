
### Requirement: HTTP handler tests SHALL run without external services

The test framework SHALL enable testing of HTTP endpoint handlers without requiring MySQL database or logger service to be running, using in-memory SQLite and HTTP mocking.

#### Scenario: Test setup creates isolated test environment
- **WHEN** a handler test is initialized
- **THEN** the framework creates an in-memory SQLite database with test schema
- **AND** the framework starts a wiremock HTTP mock server for logger service
- **AND** the framework configures the test app with mocked dependencies

#### Scenario: Tests run independently without shared state
- **WHEN** multiple handler tests execute
- **THEN** each test uses its own isolated database instance
- **AND** database changes from one test do not affect other tests
- **AND** tests can run in parallel without conflicts

#### Scenario: Test execution completes in under 1 second
- **WHEN** the full handler test suite runs
- **THEN** total execution time is under 1 second
- **AND** individual test setup takes less than 10ms

### Requirement: Tests SHALL validate HTTP request/response contracts

The test framework SHALL verify that handlers correctly process HTTP requests and generate appropriate responses for all endpoints.

#### Scenario: Test validates successful user creation
- **WHEN** test posts valid user payload to /api/users
- **THEN** handler returns HTTP 201 Created status
- **AND** response body contains user_id field with positive integer
- **AND** user is stored in test database

#### Scenario: Test validates validation errors return 400
- **WHEN** test posts invalid user payload (username too long)
- **THEN** handler returns HTTP 400 Bad Request status
- **AND** response body contains error field with "VALIDATION_ERROR"
- **AND** response body contains descriptive message field

#### Scenario: Test validates duplicate username returns 409
- **WHEN** test creates user with username "testuser"
- **AND** test attempts to create another user with username "testuser"
- **THEN** second request returns HTTP 409 Conflict status
- **AND** response body contains error field with "DUPLICATE_USERNAME"

#### Scenario: Test validates successful login
- **WHEN** test creates user with known credentials
- **AND** test posts login request with matching credentials
- **THEN** handler returns HTTP 200 OK status
- **AND** response body contains user_id matching created user

#### Scenario: Test validates invalid credentials return 401
- **WHEN** test posts login with incorrect password
- **THEN** handler returns HTTP 401 Unauthorized status
- **AND** response body contains error field with "INVALID_CREDENTIALS"

#### Scenario: Test validates user info retrieval
- **WHEN** test creates user and retrieves by ID
- **THEN** handler returns HTTP 200 OK status
- **AND** response contains all user fields (id, username, first_name, etc.)

#### Scenario: Test validates not found returns 404
- **WHEN** test requests user info for non-existent ID
- **THEN** handler returns HTTP 404 Not Found status
- **AND** response body contains error field with "USER_NOT_FOUND"

### Requirement: Tests SHALL verify logging behavior

The test framework SHALL validate that handlers correctly log events to the logger service with proper payloads and handle logger failures gracefully.

#### Scenario: Test verifies log events are sent
- **WHEN** test executes handler operation
- **THEN** wiremock verifies HTTP POST was sent to /logs endpoint
- **AND** payload includes required fields (timestamp, level, app, message)
- **AND** payload level matches operation type (info, error, warn)

#### Scenario: Test verifies log payload contains user context
- **WHEN** test executes create_user operation
- **THEN** log payload includes user field with username
- **AND** log payload includes app field with "create_user"

#### Scenario: Test verifies logger failure does not break handler
- **WHEN** wiremock is configured to return HTTP 500 error
- **AND** test executes handler operation
- **THEN** handler completes successfully
- **AND** handler returns correct HTTP status to client
- **AND** operation is not rolled back due to logger failure

#### Scenario: Test can skip logger verification
- **WHEN** test does not configure wiremock expectations
- **THEN** handler executes normally
- **AND** log calls are sent to mock server (or ignored if LOGGER_URL unset)

### Requirement: Tests SHALL simulate database error scenarios

The test framework SHALL enable testing of handler behavior when database operations fail, including connection errors and constraint violations.

#### Scenario: Test validates duplicate key handling
- **WHEN** test creates user with username "duplicate"
- **AND** test attempts second insert with same username
- **THEN** handler returns HTTP 409 Conflict
- **AND** handler does not crash or leak exceptions

#### Scenario: Test validates empty result handling
- **WHEN** test queries non-existent username for login
- **THEN** handler returns HTTP 401 Unauthorized (not 500)
- **AND** handler logs appropriate message

#### Scenario: Test validates malformed ID handling
- **WHEN** test requests user info with invalid ID format
- **THEN** handler returns HTTP 400 Bad Request
- **AND** handler does not attempt database query

### Requirement: Test helper functions SHALL simplify test authoring

The test framework SHALL provide reusable helper functions for common test setup and assertions.

#### Scenario: Helper creates test app with mocked dependencies
- **WHEN** test calls setup_test_app() helper
- **THEN** helper returns initialized actix-web test service
- **AND** helper returns test database instance
- **AND** helper returns wiremock mock server instance
- **AND** LOGGER_URL environment variable is configured

#### Scenario: Helper creates test user
- **WHEN** test calls create_test_user() helper
- **THEN** helper inserts user directly into test database
- **AND** helper returns user ID and credentials
- **AND** subsequent handler calls can authenticate as this user

#### Scenario: Helper asserts JSON response structure
- **WHEN** test calls assert_error_response() helper
- **THEN** helper validates response contains error and message fields
- **AND** helper validates error code matches expected value

### Requirement: Tests SHALL maintain compatibility with existing unit tests

The integration test suite SHALL not conflict with or replace existing unit tests, allowing both to coexist and run together.

#### Scenario: Integration tests in separate module
- **WHEN** cargo test executes
- **THEN** both main_test.rs and handler_tests.rs modules run
- **AND** unit tests complete first (faster feedback)
- **AND** integration tests follow

#### Scenario: Selective test execution is supported
- **WHEN** developer runs `cargo test main_test`
- **THEN** only unit tests execute (fast feedback loop)
- **AND** developer can run `cargo test handler_tests` for integration tests only

#### Scenario: Existing unit tests unchanged
- **WHEN** integration tests are added
- **THEN** no modifications to existing test files in main_test.rs
- **AND** existing test coverage is preserved
- **AND** test count increases but no tests are removed
