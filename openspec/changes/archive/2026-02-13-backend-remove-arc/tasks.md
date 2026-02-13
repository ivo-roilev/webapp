## 1. Refactor AppState Structure

- [x] 1.1 Remove `Arc<Database>` wrapper from `AppState.db` field, change to direct `Database` ownership
- [x] 1.2 Remove `Arc<reqwest::Client>` wrapper from `AppState.http_client` field, change to direct `reqwest::Client` ownership
- [x] 1.3 Remove `use std::sync::Arc;` import from main.rs (if no other usage remains)
- [x] 1.4 Update `main()` function to initialize `db` without `Arc::new()` wrapping
- [x] 1.5 Update `main()` function to initialize `http_client` without `Arc::new()` wrapping
- [x] 1.6 Remove `http_client.clone()` call before creating `AppState` since cloning is no longer needed

## 2. Refactor Logger Interface

- [x] 2.1 Update `dual_log` function signature to accept `client: reqwest::Client` by value instead of `client: &Arc<reqwest::Client>`
- [x] 2.2 Remove `use std::sync::Arc;` import from logger.rs
- [x] 2.3 Remove internal `let client = client.clone();` line from the async spawn block (no longer needed since function owns the value)
- [x] 2.4 Update all log macros (`log_info!`, `log_error!`, `log_warn!`, `log_debug!`) to clone client at call site with `$client.clone()`
- [x] 2.5 Remove explicit `&` from all log macro call sites in main.rs (rely on auto-deref)

## 3. Verification

- [x] 3.1 Verify all endpoint handlers compile without errors
- [x] 3.2 Verify all log macros (`log_info!`, `log_error!`, etc.) work with updated signature
- [x] 3.3 Run existing tests to ensure no behavioral changes
- [x] 3.4 Confirm handlers correctly access `state.db` and `state.http_client` through automatic dereferencing
