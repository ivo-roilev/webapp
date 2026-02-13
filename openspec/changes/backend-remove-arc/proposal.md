## Why

The current implementation uses redundant reference counting with nested `Arc` wrappers in `AppState` and logger functions. Since `web::Data<AppState>` already provides `Arc`-based sharing, wrapping individual fields (`db` and `http_client`) in additional `Arc` instances violates Rust's value semantics and single-ownership principle. This creates unnecessary complexity and goes against idiomatic Rust patterns.

## What Changes

- Remove `Arc` wrappers from `AppState` struct fields (`db` and `http_client`)
- Update `dual_log` function signature to accept `&reqwest::Client` instead of `&Arc<reqwest::Client>`
- Remove `std::sync::Arc` imports where no longer needed
- Update initialization code to use direct ownership instead of `Arc::new()`
- Change `Arc::clone()` to `.clone()` in logger's async spawn block

## Capabilities

### New Capabilities
<!-- None - this is a refactoring change -->

### Modified Capabilities
<!-- None - external behavior and requirements remain unchanged. This is purely an implementation improvement. -->

## Impact

- **Affected Code**:
  - `src/rust/main.rs`: `AppState` struct definition and initialization in `main()`
  - `src/rust/logger.rs`: `dual_log` function signature and macro implementations
- **No Breaking Changes**: External API contracts remain identical
- **No Behavioral Changes**: All endpoints and logging functionality work exactly as before
- **Benefits**: Simplified ownership model, more idiomatic Rust code, reduced indirection
