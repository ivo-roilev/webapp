## 1. Add Dependencies

- [x] 1.1 Add `reqwest = { version = "0.11", features = ["json", "rustls-tls"], default-features = false }` to `Cargo.toml` dependencies section (using rustls instead of OpenSSL)

## 2. Create Logging Module

- [x] 2.1 Create new file `src/rust/logger.rs`
- [x] 2.2 Add `LogLevel` enum with variants (Debug, Info, Warn, Error) with `#[allow(dead_code)]` attribute
- [x] 2.3 Implement `LogLevel::as_str()` method to convert to string ("debug", "info", "warn", "error")
- [x] 2.4 Implement `dual_log()` function that takes (client, level, app, user, message) parameters - NOTE: user is 4th parameter (before message)
- [x] 2.5 In `dual_log()`, format message with `[app]` prefix for local logging
- [x] 2.6 In `dual_log()`, use `log::debug!/info!/warn!/error!` based on level
- [x] 2.7 In `dual_log()`, check for `LOGGER_URL` environment variable
- [x] 2.8 In `dual_log()`, if `LOGGER_URL` exists, build JSON payload with timestamp (RFC3339), level, message, user, app
- [x] 2.9 In `dual_log()`, spawn tokio task to POST payload to `{LOGGER_URL}/logs`
- [x] 2.10 Ensure spawned HTTP task ignores errors (fire-and-forget)

## 3. Implement Logging Macros

- [x] 3.1 Create `log_info!` macro with two variants: (client, app, user, msg) and (client, app, user, fmt, args...) - NOTE: user is 4th positional parameter
- [x] 3.2 Create `log_error!` macro with same two variants
- [x] 3.3 Create `log_warn!` macro with same two variants
- [x] 3.4 Create `log_debug!` macro with same two variants
- [x] 3.5 Export macros with `#[macro_export]` attribute
- [x] 3.6 Convert empty string user parameter to None internally for optional user context

## 4. Update Application State

- [x] 4.1 Add `mod logger;` declaration to `src/rust/main.rs`
- [x] 4.2 Add `http_client: Arc<reqwest::Client>` field to `AppState` struct
- [x] 4.3 In `main()` function, create `Arc::new(reqwest::Client::new())` before server startup
- [x] 4.4 Pass `http_client` to `AppState` when creating `web::Data`
- [x] 4.5 Remove unused `use log::info;` import

## 5. Migrate create_user Handler

- [x] 5.1 Replace `info!("Creating new user: {}", payload.username)` with `log_info!(&state.http_client, "create_user", payload.username, "Creating new user")`
- [x] 5.2 Replace `info!("User created successfully with ID: {}", user_id)` with `log_info!(&state.http_client, "create_user", payload.username, "User created successfully with ID: {}", user_id)`
- [x] 5.3 Replace `info!("Username {} already exists", payload.username)` with `log_info!(&state.http_client, "create_user", payload.username, "Username already exists")`
- [x] 5.4 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "create_user", payload.username, "Database connection error")`
- [x] 5.5 Replace `log::error!("Error creating user: {:?}", e)` with `log_error!(&state.http_client, "create_user", payload.username, "Error creating user: {:?}", e)`

## 6. Migrate login Handler

- [x] 6.1 Replace `info!("Login attempt for user: {}", payload.username)` with `log_info!(&state.http_client, "login_user", payload.username, "Login attempt")`
- [x] 6.2 Replace `info!("Successful login for user: {}", payload.username)` with `log_info!(&state.http_client, "login_user", payload.username, "Successful login")`
- [x] 6.3 Replace `info!("Invalid password for user: {}", payload.username)` with `log_info!(&state.http_client, "login_user", payload.username, "Invalid password")`
- [x] 6.4 Replace `info!("User not found during login: {}", payload.username)` with `log_info!(&state.http_client, "login_user", payload.username, "User not found during login")`
- [x] 6.5 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "login_user", payload.username, "Database connection error")`
- [x] 6.6 Replace `log::error!("Error during login: {:?}", e)` with `log_error!(&state.http_client, "login_user", payload.username, "Error during login: {:?}", e)`

## 7. Migrate get_user_info Handler

- [x] 7.1 Replace `info!("Fetching user info for ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", user_id, "Fetching user info")` - NOTE: Using user_id when username not yet available
- [x] 7.2 Replace `info!("User info retrieved for ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", username, "User info retrieved for ID: {}", user_id)` - NOTE: After DB fetch, user.username is available, so use it instead of user_id
- [x] 7.3 Replace `info!("User not found with ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", user_id, "User not found")`
- [x] 7.4 Replace `info!("Invalid user_id (non-positive): {}", user_id_str)` with `log_info!(&state.http_client, "get_user_info", user_id_str, "Invalid user_id (non-positive)")`
- [x] 7.5 Replace `info!("Invalid user_id format: {}", user_id_str)` with `log_info!(&state.http_client, "get_user_info", user_id_str, "Invalid user_id format")`
- [x] 7.6 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "get_user_info", "", "Database connection error")`
- [x] 7.7 Replace `log::error!("Error fetching user: {:?}", e)` with `log_error!(&state.http_client, "get_user_info", user_id, "Error fetching user: {:?}", e)`

## 8. Migrate main() Startup Logging

- [x] 8.1 Replace `log::error!("Failed to initialize database: {:?}", e)` with `log_error!(&http_client, "main", "SYSTEM", "Failed to initialize database: {:?}", e)` - NOTE: Using "SYSTEM" for system-level operations
- [x] 8.2 Replace `info!("Starting HTTP server on {}", bind_addr)` with `log_info!(&http_client, "main", "SYSTEM", "Starting HTTP server on {}", bind_addr)`

## 9. Update Configuration

- [x] 9.1 Add `LOGGER_URL=http://localhost:9090` to `.env` file (or `.env.example`)
- [x] 9.2 Document `LOGGER_URL` as optional configuration in relevant README/docs

## 10. Testing

- [x] 10.1 Run `cargo test` to verify compilation and all 23 tests pass
- [ ] 10.2 Start logger service (Go service) on localhost:9090
- [ ] 10.3 Set `LOGGER_URL=http://localhost:9090` in `.env`
- [ ] 10.4 Start webapp with `cargo run`
- [ ] 10.5 Verify startup log appears in both stdout AND logger service file with user="SYSTEM"
- [ ] 10.6 Test POST /api/users (create user) - verify logs in both destinations with username field
- [ ] 10.7 Test POST /api/login - verify logs in both destinations with username field
- [ ] 10.8 Test GET /api/users/{id} - verify logs in both destinations (username when user found, user_id otherwise)
- [ ] 10.9 Test with invalid credentials - verify error logs in both destinations
- [ ] 10.10 Stop logger service, restart webapp, verify local logs still work (graceful degradation)
- [ ] 10.11 Unset `LOGGER_URL`, restart webapp, verify only local logs (no HTTP attempts)
- [ ] 10.12 Check logger service log files for correct timestamp, level, app, user, message fields
- [ ] 10.13 Verify messages don't redundantly repeat user information (username/user_id captured in user field)
- [ ] 10.14 Verify system-level operations (startup, database init errors) use user="SYSTEM"
