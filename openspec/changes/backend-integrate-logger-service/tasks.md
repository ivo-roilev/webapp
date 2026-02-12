## 1. Add Dependencies

- [ ] 1.1 Add `reqwest = { version = "0.11", features = ["json"] }` to `Cargo.toml` dependencies section

## 2. Create Logging Module

- [ ] 2.1 Create new file `src/rust/logger.rs`
- [ ] 2.2 Add `LogLevel` enum with variants (Debug, Info, Warn, Error)
- [ ] 2.3 Implement `LogLevel::as_str()` method to convert to string ("debug", "info", "warn", "error")
- [ ] 2.4 Implement `dual_log()` function that takes (client, level, app, message, user) parameters
- [ ] 2.5 In `dual_log()`, format message with `[app]` prefix for local logging
- [ ] 2.6 In `dual_log()`, use `log::debug!/info!/warn!/error!` based on level
- [ ] 2.7 In `dual_log()`, check for `LOGGER_URL` environment variable
- [ ] 2.8 In `dual_log()`, if `LOGGER_URL` exists, build JSON payload with timestamp (RFC3339), level, message, user, app
- [ ] 2.9 In `dual_log()`, spawn tokio task to POST payload to `{LOGGER_URL}/logs`
- [ ] 2.10 Ensure spawned HTTP task ignores errors (fire-and-forget)

## 3. Implement Logging Macros

- [ ] 3.1 Create `log_info!` macro with three variants: (client, app, msg), (client, app, msg, user = val), (client, app, fmt, args...)
- [ ] 3.2 Create `log_error!` macro with same three variants
- [ ] 3.3 Create `log_warn!` macro with same three variants
- [ ] 3.4 Create `log_debug!` macro with same three variants
- [ ] 3.5 Export macros with `#[macro_export]` attribute

## 4. Update Application State

- [ ] 4.1 Add `mod logger;` declaration to `src/rust/main.rs`
- [ ] 4.2 Add `http_client: Arc<reqwest::Client>` field to `AppState` struct
- [ ] 4.3 In `main()` function, create `Arc::new(reqwest::Client::new())` before server startup
- [ ] 4.4 Pass `http_client` to `AppState` when creating `web::Data`

## 5. Migrate create_user Handler

- [ ] 5.1 Replace `info!("Creating new user: {}", payload.username)` with `log_info!(&state.http_client, "create_user", "Creating new user: {}", payload.username, user = payload.username)`
- [ ] 5.2 Replace `info!("User created successfully with ID: {}", user_id)` with `log_info!(...)`
- [ ] 5.3 Replace `info!("Username {} already exists", payload.username)` with `log_info!(...)`
- [ ] 5.4 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "create_user", "Database connection error")`
- [ ] 5.5 Replace `log::error!("Error creating user: {:?}", e)` with `log_error!(&state.http_client, "create_user", "Error creating user: {:?}", e, user = payload.username)`

## 6. Migrate login_user Handler

- [ ] 6.1 Replace `info!("Login attempt for user: {}", payload.username)` with `log_info!(&state.http_client, "login_user", "Login attempt for user: {}", payload.username, user = payload.username)`
- [ ] 6.2 Replace `info!("Successful login for user: {}", payload.username)` with `log_info!(...)`
- [ ] 6.3 Replace `info!("Invalid password for user: {}", payload.username)` with `log_info!(...)`
- [ ] 6.4 Replace `info!("User not found during login: {}", payload.username)` with `log_info!(...)`
- [ ] 6.5 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "login_user", "Database connection error")`
- [ ] 6.6 Replace `log::error!("Error during login: {:?}", e)` with `log_error!(&state.http_client, "login_user", "Error during login: {:?}", e, user = payload.username)`

## 7. Migrate get_user_info Handler

- [ ] 7.1 Replace `info!("Fetching user info for ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", "Fetching user info for ID: {}", user_id)`
- [ ] 7.2 Replace `info!("User info retrieved for ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", "User info retrieved for ID: {}", user_id)`
- [ ] 7.3 Replace `info!("User not found with ID: {}", user_id)` with `log_info!(&state.http_client, "get_user_info", "User not found with ID: {}", user_id)`
- [ ] 7.4 Replace `info!("Invalid user_id (non-positive): {}", user_id_str)` with `log_info!(&state.http_client, "get_user_info", "Invalid user_id (non-positive): {}", user_id_str)`
- [ ] 7.5 Replace `info!("Invalid user_id format: {}", user_id_str)` with `log_info!(&state.http_client, "get_user_info", "Invalid user_id format: {}", user_id_str)`
- [ ] 7.6 Replace `log::error!("Database connection error")` with `log_error!(&state.http_client, "get_user_info", "Database connection error")`
- [ ] 7.7 Replace `log::error!("Error fetching user: {:?}", e)` with `log_error!(&state.http_client, "get_user_info", "Error fetching user: {:?}", e)`

## 8. Migrate main() Startup Logging

- [ ] 8.1 Replace `log::error!("Failed to initialize database: {:?}", e)` with `log_error!(&http_client, "main", "Failed to initialize database: {:?}", e)`
- [ ] 8.2 Replace `info!("Starting HTTP server on {}", bind_addr)` with `log_info!(&http_client, "main", "Starting HTTP server on {}", bind_addr)`

## 9. Update Configuration

- [ ] 9.1 Add `LOGGER_URL=http://localhost:9090` to `.env` file (or `.env.example`)
- [ ] 9.2 Document `LOGGER_URL` as optional configuration in relevant README/docs

## 10. Testing

- [ ] 10.1 Run `cargo build` to verify compilation with new dependencies
- [ ] 10.2 Start logger service (Go service) on localhost:9090
- [ ] 10.3 Set `LOGGER_URL=http://localhost:9090` in `.env`
- [ ] 10.4 Start webapp with `cargo run`
- [ ] 10.5 Verify startup log appears in both stdout AND logger service file
- [ ] 10.6 Test POST /api/users (create user) - verify logs in both destinations
- [ ] 10.7 Test POST /api/login - verify logs in both destinations
- [ ] 10.8 Test GET /api/users/{id} - verify logs in both destinations
- [ ] 10.9 Test with invalid credentials - verify error logs in both destinations
- [ ] 10.10 Stop logger service, restart webapp, verify local logs still work (graceful degradation)
- [ ] 10.11 Unset `LOGGER_URL`, restart webapp, verify only local logs (no HTTP attempts)
- [ ] 10.12 Check logger service log files for correct timestamp, level, app, user, message fields
