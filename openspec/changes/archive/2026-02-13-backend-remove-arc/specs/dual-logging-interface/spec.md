## MODIFIED Requirements

### Requirement: Dual logging interface accepts HTTP client for remote logging

The dual logging system SHALL accept an HTTP client instance to enable remote logging capabilities while maintaining identical external behavior.

**Implementation Note**: This requirement's implementation has been refactored to use idiomatic Rust ownership patterns (pass-by-value instead of explicit Arc wrappers), but the functional behavior remains unchanged.

#### Scenario: Logger accepts HTTP client and performs both local and remote logging

- **WHEN** the dual_log function is called with an HTTP client, log level, app name, user info, and message
- **THEN** the system logs the message locally via the standard logging framework
- **AND** the system sends the log entry to the remote logging service if LOGGER_URL is configured
- **AND** the remote logging operation runs asynchronously without blocking the caller
- **AND** failures in remote logging do not affect local logging or application functionality

#### Scenario: Logger operates identically regardless of ownership model

- **WHEN** the logger is invoked from any handler with state.http_client
- **THEN** the logging behavior (local + remote) executes exactly as before refactoring
- **AND** all log macros (log_info!, log_error!, log_warn!, log_debug!) continue to work identically
- **AND** no external API contracts or behaviors are modified
