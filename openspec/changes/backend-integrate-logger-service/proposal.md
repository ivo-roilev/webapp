## Why

The webapp currently logs to stdout via `env_logger`, which is adequate for local development but lacks centralized aggregation, searchability, and persistence. The existing logger service (Go-based HTTP service with date-based file routing) provides structured log storage and querying capabilities. Integrating the Rust webapp with this logger service enables centralized log management, security auditing (login attempts, invalid passwords), and operational visibility across the application stack.

## What Changes

- Create new `src/rust/logger.rs` module with dual-logging functionality (local stdout + remote HTTP service)
- Add `log_info!`, `log_warn!`, `log_error!`, `log_debug!` macros for ergonomic logging
- Add shared `reqwest::Client` to `AppState` for connection pooling to logger service
- Replace all existing `log::info!()` and `log::error!()` calls (~21 locations) with dual-logging macros
- Add `LOGGER_URL` environment variable configuration (optional, graceful degradation if not set)
- Add `reqwest` dependency with `json` and `rustls-tls` features to `Cargo.toml` (pure Rust TLS, no OpenSSL dependency)
- Async fire-and-forget HTTP POSTs to logger service (non-blocking, no error handling - logs lost if service down)
- Include function name as `app` field in log payloads for operation-level filtering
- Maintain consistent log format between local stdout and remote service
- Support all log levels (debug/info/warn/error) based on `RUST_LOG` configuration

## Capabilities

### New Capabilities
- `dual-logging-interface`: Dual-output logging system that sends log events to both local stdout (via env_logger) and remote logger service (via HTTP POST) with shared HTTP client pooling and macro-based API

### Modified Capabilities
<!-- No existing capabilities modified - this is pure addition -->

## Impact

- **Code**: All handler functions in `src/rust/main.rs` (~21 log call sites modified)
- **New Module**: `src/rust/logger.rs` (logging abstraction, macros, dual_log function)
- **Dependencies**: `Cargo.toml` (add `reqwest = { version = "0.11", features = ["json", "rustls-tls"], default-features = false }`)
- **Configuration**: `.env` file (add `LOGGER_URL=http://localhost:9090`, optional)
- **App State**: `AppState` struct (add `http_client: Arc<reqwest::Client>` field)
- **Runtime behavior**: Asynchronous HTTP POST spawned for each log event (non-blocking, best-effort delivery)
- **Deployment**: Requires logger service running if centralized logging desired (graceful degradation if unavailable)
- **Performance**: Minimal overhead - shared HTTP client reuses connections, spawned tasks don't block handlers
- **Security**: Logs include sensitive events (invalid password attempts, user lookups) for audit trail
