## Context

The current codebase uses `web::Data<AppState>` to share application state (database connection pool and HTTP client) across Actix Web request handlers. `web::Data<T>` internally wraps `T` in an `Arc`, providing thread-safe shared ownership.

However, the `AppState` struct's fields are themselves wrapped in additional `Arc` instances:
```rust
struct AppState {
    db: Arc<Database>,
    http_client: Arc<reqwest::Client>,
}
```

This creates double Arc-wrapping: `Arc<AppState>` (from `web::Data`) containing `Arc<Database>` and `Arc<reqwest::Client>`. Similarly, the `dual_log` function accepts `&Arc<reqwest::Client>` and clones it for async operations.

**Current State:**
- Nested Arc wrappers create unnecessary reference counting overhead
- Code doesn't follow Rust's idiomatic single-ownership principle
- The pattern resembles Java-style reference counting rather than Rust value semantics

**Constraint:** The refactoring must maintain identical external behavior and not break existing handler signatures.

## Goals / Non-Goals

**Goals:**
- Eliminate redundant Arc wrappers from `AppState` fields
- Use direct ownership with single Arc layer (provided by `web::Data`)
- Update logger interface to accept references instead of Arc wrappers
- Follow idiomatic Rust ownership patterns

**Non-Goals:**
- Changing any external API behavior or endpoint contracts
- Modifying database or HTTP client functionality
- Performance optimization (this is about code clarity and idiomaticity)
- Changing the `web::Data` extraction pattern in handlers

## Decisions

### Decision 1: Remove Inner Arc Wrappers from AppState

**Choice:** Change `AppState` to use direct ownership:
```rust
struct AppState {
    db: Database,
    http_client: reqwest::Client,
}
```

**Rationale:**
- `web::Data<AppState>` already provides Arc-based sharing
- Handlers receive `web::Data<AppState>` which derefs to `&AppState`
- When handlers access `state.db`, they automatically get `&Database` through deref coercion
- Eliminates double reference counting with no loss of functionality

**Alternative Considered:** Keep Arc wrappers and document as intentional pattern
- **Rejected:** Creates complexity without benefit and violates Rust idioms

### Decision 2: Update Logger to Take Ownership

**Choice:** Change `dual_log` signature from `client: &Arc<reqwest::Client>` to `client: reqwest::Client` (by value)

**Rationale:**
- Logger is used by default in nearly all requests, so cloning always happens
- Clone cost is negligible compared to HTTP request overhead
- Pass-by-value semantics make ownership transfer explicit and clear
- Eliminates conditional cloning logic inside the function - ownership already transferred
- `reqwest::Client::clone()` is cheap (internally Arc-based, just increments reference count)
- More idiomatic: function that needs ownership should take ownership

**Alternative Considered:** Accept reference and clone internally only when needed
- **Rejected:** Premature optimization. Logger is always used, and cloning is always needed for async spawn. The clarity of pass-by-value outweighs the negligible performance difference.

### Decision 3: Clone at Call Sites via Macros

**Choice:** Have log macros (`log_info!`, `log_error!`, etc.) perform `.clone()` and pass owned value to `dual_log`

**Rationale:**
- Centralizes cloning logic in one place (the macro)
- Call sites use auto-deref: `log_info!(state.http_client, ...)` is clean and ergonomic
- Explicit about cost: the macro shows `.clone()` is happening
- No need for `&` at call sites - Rust's auto-deref handles `state.http_client` → `&reqwest::Client`
- Consistent pattern across all log levels

## Risks / Trade-offs

**Risk:** Breaking changes if other code depends on Arc being part of the type signature
**Mitigation:** This is internal refactoring only. All handler call sites use `&AppState` through `web::Data` extraction, which remains unchanged.

**Risk:** Confusion about when cloning occurs
**Mitigation:** The clone happens explicitly in the macro (visible in code), and the cost is negligible. Since logging is ubiquitous, optimizing away the clone in rare non-logging scenarios would be premature optimization.

**Trade-off:** Clone at every log call vs. conditional clone inside function
**Rationale:** Logging is used by default in nearly all requests. The simplicity of pass-by-value ownership semantics outweighs any theoretical performance benefit from conditional cloning. The clone cost (~100ns for Arc increment) is multiple orders of magnitude smaller than any HTTP operation.

**Trade-off:** Slightly less explicit about sharing semantics
**Benefit:** More idiomatic code that follows Rust's ownership model. The sharing is still present (via the outer `web::Data<AppState>` and internal Arc in `reqwest::Client`), just not redundantly expressed at multiple layers.
