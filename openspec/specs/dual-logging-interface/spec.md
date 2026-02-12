## ADDED Requirements

### Requirement: Logging function SHALL send events to both local stdout and remote HTTP service

The dual logging system SHALL output log events to two destinations simultaneously:
1. Local stdout via Rust's `log` crate and `env_logger` backend
2. Remote HTTP POST to logger service endpoint (if `LOGGER_URL` environment variable is set)

#### Scenario: Log with logger service configured
- **WHEN** `LOGGER_URL` environment variable is set to `http://localhost:9090`
- **AND** application calls dual logging function (e.g., `log_info!(&client, "create_user", "message")`)
- **THEN** log event SHALL appear in stdout with format `[function_name] message`
- **AND** log event SHALL be sent via HTTP POST to `http://localhost:9090/logs` with JSON payload

#### Scenario: Log without logger service configured
- **WHEN** `LOGGER_URL` environment variable is not set or empty
- **AND** application calls dual logging function
- **THEN** log event SHALL appear in stdout with format `[function_name] message`
- **AND** no HTTP request SHALL be made (local logging only)

### Requirement: Logging SHALL be non-blocking and SHALL NOT affect request latency

The dual logging system SHALL use asynchronous fire-and-forget delivery for HTTP requests. Logging failures SHALL NOT block handler functions or affect user responses.

#### Scenario: Successful async log delivery
- **WHEN** dual logging function is called
- **THEN** function SHALL return immediately without waiting for HTTP response
- **AND** HTTP POST SHALL be executed in spawned tokio task

#### Scenario: Logger service unavailable
- **WHEN** dual logging function sends HTTP POST
- **AND** logger service is not reachable or returns error
- **THEN** HTTP error SHALL be ignored (no retry, no error propagation)
- **AND** handler function SHALL continue normally
- **AND** user request SHALL complete successfully

### Requirement: Logging SHALL use shared HTTP client for connection pooling

The dual logging system SHALL reuse a single `reqwest::Client` instance stored in application state to minimize TCP handshake overhead.

#### Scenario: HTTP client initialized at startup
- **WHEN** application starts
- **THEN** a single `Arc<reqwest::Client>` SHALL be created
- **AND** client SHALL be stored in `AppState` struct
- **AND** client SHALL be passed to all dual logging function calls

#### Scenario: Multiple log calls reuse connection
- **WHEN** multiple log events are sent to logger service
- **THEN** HTTP client SHALL reuse TCP connections when possible
- **AND** multiple handshakes SHALL be avoided for sequential requests

### Requirement: Log payload SHALL include timestamp, level, message, app, and optional user

The JSON payload sent to logger service SHALL conform to the logger service API schema.

#### Scenario: Log with user context
- **WHEN** dual logging is called with user parameter (e.g., `user = username`)
- **THEN** HTTP POST payload SHALL include:
  - `timestamp`: RFC3339 UTC timestamp (e.g., `"2026-02-12T10:15:30Z"`)
  - `level`: Log level string (`"debug"`, `"info"`, `"warn"`, or `"error"`)
  - `message`: The formatted log message
  - `app`: Function/operation name passed by caller
  - `user`: The user identifier string

#### Scenario: Log without user context
- **WHEN** dual logging is called without user parameter
- **THEN** HTTP POST payload SHALL include timestamp, level, message, app fields
- **AND** `user` field SHALL be omitted or null

### Requirement: Logging SHALL support debug, info, warn, and error levels

The dual logging system SHALL provide macros for all standard log levels: `log_debug!`, `log_info!`, `log_warn!`, `log_error!`.

#### Scenario: Info level logging
- **WHEN** code calls `log_info!(&client, "login_user", "Login successful")`
- **THEN** local stdout SHALL show `[INFO] [login_user] Login successful`
- **AND** remote payload SHALL have `"level": "info"`

#### Scenario: Error level logging
- **WHEN** code calls `log_error!(&client, "create_user", "Database error: {}", err)`
- **THEN** local stdout SHALL show `[ERROR] [create_user] Database error: ...`
- **AND** remote payload SHALL have `"level": "error"`

#### Scenario: Debug level logging
- **WHEN** `RUST_LOG=debug` is set
- **AND** code calls `log_debug!(&client, "get_user_info", "Fetching user ID: {}", id)`
- **THEN** local stdout SHALL show `[DEBUG] [get_user_info] Fetching user ID: ...`
- **AND** remote payload SHALL have `"level": "debug"`

### Requirement: Macros SHALL support format strings and optional user parameter

The dual logging macros SHALL accept format strings like standard Rust `format!()` and SHALL support optional `user = value` syntax.

#### Scenario: Format string with arguments
- **WHEN** macro is called as `log_info!(&client, "create_user", "User created with ID: {}", user_id)`
- **THEN** message SHALL be formatted with user_id value interpolated

#### Scenario: User parameter provided
- **WHEN** macro is called as `log_info!(&client, "login_user", "Login attempt", user = username)`
- **THEN** HTTP payload SHALL include `"user": <username value>`

#### Scenario: Format string and user parameter combined
- **WHEN** macro is called as `log_info!(&client, "login_user", "Login for: {}", username, user = username)`
- **THEN** message SHALL be formatted with username
- **AND** HTTP payload SHALL include `"user": <username value>`

### Requirement: Function name SHALL be passed as app field for log filtering

The dual logging system SHALL require callers to explicitly pass the function or operation name as a string parameter, which SHALL be sent as the `app` field in the HTTP payload and included in local stdout format.

#### Scenario: Function name in log output
- **WHEN** dual logging is called with app parameter `"create_user"`
- **THEN** local stdout SHALL include `[create_user]` in the log line
- **AND** HTTP payload SHALL have `"app": "create_user"`

#### Scenario: Operation-level log filtering
- **WHEN** logger service receives logs with different app values
- **THEN** logs SHALL be filterable by operation name (e.g., `grep '[create_user]' logs/2026-02-12.log`)
