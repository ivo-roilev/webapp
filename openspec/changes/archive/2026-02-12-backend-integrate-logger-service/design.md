## Context

The Rust webapp currently uses the standard Rust `log` crate with `env_logger` backend, outputting to stdout. This is adequate for local development but lacks:
- Centralized log aggregation across services
- Persistence beyond process lifetime
- Searchability and filtering (by operation, user, date)
- Audit trail for security events (login attempts, invalid passwords)

A separate Go-based logger service already exists at `../logger/` that accepts log events via HTTP POST and writes them to date-based files with structured fields. This design integrates the Rust webapp with that logger service while maintaining local stdout logging for development debugging.

**Current logging pattern:**
```rust
log::info!("Login attempt for user: {}", username);
log::error!("Error creating user: {:?}", e);
```

**Target pattern:**
```rust
log_info!(&client, "login_user", username, "Login attempt");
log_error!(&client, "create_user", username, "Error creating user: {:?}", e);
```

Note: User field is 4th positional parameter (after app, before message). When username is available, use it instead of user_id for better log readability.

## Goals / Non-Goals

**Goals:**
- Dual-output logging (local stdout + remote HTTP service)
- Non-blocking async delivery (fire-and-forget, no impact on request latency)
- Shared HTTP client for connection pooling (minimize handshake overhead)
- Macro-based ergonomic API similar to existing `log::` macros
- Function name as `app` field for operation-level log filtering
- Consistent log format between local and remote outputs
- Support all log levels (debug/info/warn/error)
- Graceful degradation if logger service unavailable

**Non-Goals:**
- Guaranteed log delivery (logs lost if service down is acceptable)
- Structured field extraction (keep formatted messages for now)
- Retry logic or buffering
- Local file backup if remote fails
- mTLS or authentication to logger service
- Distributed tracing / correlation IDs (future enhancement)
- Batching or rate limiting

## Decisions

### Decision 1: Dual Logging (Local + Remote)

**Choice:** Keep existing stdout logging AND add remote HTTP logging

**Rationale:**
- Local logs essential for development debugging (`cargo run` output)
- Remote logs required for centralized aggregation and audit trail
- Both outputs serve different purposes, not redundant
- Minimal code complexity to support both

**Alternatives Considered:**
- Remote only: Rejected - breaks development workflow, requires logger service to be running
- Local only: Rejected - doesn't solve centralization problem

### Decision 2: Shared HTTP Client with Connection Pooling

**Choice:** Create `Arc<reqwest::Client>` once at startup, store in `AppState`, pass to logging functions

**Rationale:**
- Reduces TCP handshake overhead (client reuses connections)
- Logger service is on localhost:9090, short-lived connections wasteful
- `reqwest::Client` is designed to be reused (internally manages connection pool)
- Minimal memory overhead (one client per app instance)

**Alternatives Considered:**
- Create fresh client per log call: Rejected - unnecessary handshake overhead, worse performance
- Global static client: Rejected - harder to test, less flexible than dependency injection via AppState

### Decision 3: Async Fire-and-Forget (tokio::spawn, no await)

**Choice:** Logging function spawns HTTP POST task and returns immediately, no error handling

**Rationale:**
- **Non-blocking:** Handler functions don't wait for HTTP response
- **No latency impact:** User requests not delayed by logging
- **Simple failure model:** If logger service down, logs lost (acceptable per requirements)
- **No error propagation:** Logging should never cause handler to fail

**Alternatives Considered:**
- Synchronous blocking HTTP POST: Rejected - adds latency to every request, logger failure blocks users
- Async with `.await`: Rejected - still waits for network, adds complexity to call sites
- Channel-based background worker: Rejected - adds complexity, no retry needed per goals

### Decision 4: Macro-Based API

**Choice:** Implement `log_debug!`, `log_info!`, `log_warn!`, `log_error!` macros

**Rationale:**
- **Ergonomic:** Similar to existing `log::` macros developers already use
- **Flexible:** Can support optional `user = value` syntax
- **String formatting:** Can use format string patterns like `"msg: {}"` naturally
- **Reversible:** Can revert to functions later if macros become complex

**Alternatives Considered:**
- Function-only API: `dual_log(client, level, app, msg, user)` - Rejected - verbose, awkward format string handling
- Extend `log` crate with custom backend: Rejected - complex, wouldn't support structured user field easily

**Macro signature pattern:**
```rust
log_info!(&client, "create_user", "SYSTEM", "User created");  // System-level operation
log_info!(&client, "login_user", username, "Login attempt");  // With username
log_info!(&client, "get_user_info", user_id, "Fetching user info");  // With user_id when username unavailable
```

Note: User parameter is always 4th position. Use `"SYSTEM"` for system-level operations (startup, shutdown, etc.), empty string `""` when no user context available but operation is not system-level. Prefer username over user_id when both are available.

### Decision 5: Function Name as `app` Field

**Choice:** Require caller to pass function/operation name (e.g., `"create_user"`) as string literal

**Rationale:**
- **Queryable logs:** Can filter logger service files by operation (`grep '[create_user]'`)
- **Better than binary name:** `"rust_user_service"` too broad, not useful for filtering
- **Explicit over magic:** Manual passing is clear, avoids macro complexity of extracting function name
- **Maps to business operations:** `create_user`, `login_user`, `get_user_info` are semantic

**Alternatives Considered:**
- Use module path or binary name: Rejected - not useful for operation-level filtering
- Extract function name via macro magic (`function_name!()` crate): Rejected - adds dependency, harder to debug

### Decision 6: Simple Message Format (No Field Extraction)

**Choice:** Keep existing formatted string messages, don't extract structured fields yet

**Rationale:**
- **Simpler migration:** Can replace `info!(...)` calls with minimal changes
- **Lower complexity:** Avoid need to rewrite all log calls to extract fields
- **Future enhancement:** Can add field extraction incrementally without breaking API

**Example:**
```rust
// Current
info!("User created successfully with ID: {}", user_id);

// New (implemented)
log_info!(&client, "create_user", username, "User created successfully with ID: {}", user_id);

// Future optimization (avoiding redundancy - IMPLEMENTED)
log_info!(&client, "create_user", username, "User created successfully");
// User info is captured in the user field, no need to repeat in message
```

### Decision 7: Optional Logger Service (Graceful Degradation)

**Choice:** Check for `LOGGER_URL` env var; if missing, only log locally

**Rationale:**
- **Development flexibility:** Logger service not required for local testing
- **Deployment flexibility:** Can run webapp standalone if needed
- **Fail-safe:** Missing config doesn't break application startup

**Implementation:**
```rust
if let Ok(logger_url) = std::env::var("LOGGER_URL") {
    // spawn HTTP POST task
}
// Always proceeds, even if LOGGER_URL not set
```

### Decision 8: All-at-Once Migration

**Choice:** Convert all ~21 log call sites in single change

**Rationale:**
- **Consistency:** No mixed old/new patterns in codebase
- **Complete deployment:** All operations logged to service from day one
- **Simpler review:** One PR, one pattern
- **Small codebase:** 21 call sites manageable in one change

**Alternatives Considered:**
- Gradual migration (per-handler): Rejected - creates inconsistent logging, partial value from logger service

## Risks / Trade-offs

**Risk:** Logger service down → logs lost
**Mitigation:** Acceptable per requirements. Local stdout logs still work. Could add file backup later if needed.

**Risk:** High log volume → spawns many tokio tasks
**Mitigation:** Each task is lightweight (HTTP POST). Tokio designed for many concurrent tasks. Not expecting >100 QPS in this project.

**Trade-off:** Manual function name passing → potential typos or inconsistency
**Mitigation:** Could add tests that validate function names match actual function. For now, code review sufficient.

**Trade-off:** Macros harder to debug than functions
**Mitigation:** Keep macros simple (just argument parsing). Can revert to functions if needed. Rust-analyzer shows macro expansions.

**Trade-off:** No guaranteed delivery → audit trail gaps
**Mitigation:** Acceptable for dev/test project. Production would add buffering/retry or use sidecar pattern.

## Migration Plan

### Implementation Steps:
1. Add `reqwest` dependency to `Cargo.toml`
2. Create `src/rust/logger.rs` module with `dual_log()` function and macros
3. Add `http_client: Arc<reqwest::Client>` to `AppState` struct
4. Initialize client in `main()` before server start
5. Replace all `log::info!()` calls → `log_info!()`
6. Replace all `log::error!()` calls → `log_error!()`
7. Add `LOGGER_URL` to `.env` example/documentation

### Testing:
- Verify local logs still output correctly
- Start logger service on localhost:9090
- Set `LOGGER_URL=http://localhost:9090`
- Exercise all handlers (create user, login, get user info)
- Verify logs appear in both stdout AND `logs/YYYY-MM-DD.log`
- Test without `LOGGER_URL` set → local logs only, no errors

### Rollback Strategy:
- Remove `LOGGER_URL` from `.env` → disables remote logging
- Revert code changes → falls back to `env_logger` only
- No database changes, no schema impact

## Open Questions

None - design decisions finalized during exploration phase.
