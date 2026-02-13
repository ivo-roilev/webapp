## Purpose

This specification defines ownership patterns for shared resources in the Rust backend application, ensuring idiomatic Rust value semantics are used instead of redundant reference counting patterns.

## Requirements

### Requirement: Application state and logger use idiomatic Rust ownership patterns

The backend application SHALL manage shared resources (database connections, HTTP clients) using idiomatic Rust ownership patterns with single-layer Arc wrapping, eliminating redundant reference counting while maintaining identical external behavior.

**Implementation Note**: This requirement's implementation has been refactored to remove redundant Arc wrappers from AppState and update the dual_log function to accept ownership by value, following Rust's value semantics rather than Java-style multi-layer reference counting.

#### Scenario: AppState fields use direct ownership with web::Data providing Arc wrapper

- **WHEN** handlers access application state via web::Data<AppState>
- **THEN** the AppState struct contains Database and reqwest::Client by direct ownership (not Arc-wrapped)
- **AND** web::Data provides the single Arc layer for thread-safe sharing
- **AND** handlers automatically deref to &Database and &reqwest::Client through field access
- **AND** all endpoint behavior remains identical to pre-refactoring implementation

#### Scenario: Logger accepts HTTP client by value following pass-by-value ownership semantics

- **WHEN** the dual_log function is called with an HTTP client, log level, app name, user info, and message
- **THEN** the function accepts client: reqwest::Client by value (ownership transferred)
- **AND** the system logs the message locally via the standard logging framework
- **AND** the system sends the log entry to the remote logging service if LOGGER_URL is configured
- **AND** the remote logging operation runs asynchronously without blocking the caller
- **AND** log macros handle cloning at call sites (state.http_client auto-derefs to reference, then cloned)

#### Scenario: All endpoint handlers work identically with refactored ownership model

- **WHEN** any endpoint handler (create_user, login, get_user_info) is invoked
- **THEN** logging behavior executes exactly as before refactoring
- **AND** all log macros (log_info!, log_error!, log_warn!, log_debug!) produce identical output
- **AND** no external API contracts or behaviors are modified
- **AND** all unit tests pass without modification
