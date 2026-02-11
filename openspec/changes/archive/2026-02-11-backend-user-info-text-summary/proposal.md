## Why

The current user info endpoint returns raw database fields as JSON, requiring frontend parsing and formatting. This exposes the underlying structure and unnecessary information to the end-user. A human-readable text summary provides a cleaner, controlled, more maintainable interface for displaying user profiles.

## What Changes

- **BREAKING**: Modify `/api/users/{user_id}` endpoint to return plain text summary instead of JSON object
- Implement text formatting logic that constructs friendly summaries from database fields
- Change response Content-Type from `application/json` to `text/plain`
- Convert error responses to plain text format

## Capabilities

### New Capabilities
None

### Modified Capabilities
- `get-user-info-endpoint`: Response format changes from structured JSON to human-readable text summary
- `ui-user-info`: Frontend must handle plain text response instead of JSON parsing

## Impact

**Backend (Rust service)**
- `src/rust/main.rs`: Modify `get_user_info` handler and `UserInfoResponse` formatting
- Response type changes from JSON to text/plain

**Frontend (React)**
- `src/web/src/`: User info display logic may need adjustment for text rendering

**API Contract**
- Breaking change to `/api/users/{user_id}` response format
- Clients expecting JSON structure will break
