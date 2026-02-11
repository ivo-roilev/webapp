## Context

The `/api/users/{user_id}` endpoint currently returns a JSON object with all user database fields (id, username, first_name, last_name, email, title, hobby, timestamps). This exposes raw database structure to clients and requires frontend formatting logic.

The database schema provides:
- **Required**: username (max 16 chars)
- **Optional**: first_name, last_name, email, title, hobby (each max 255 chars)

Current implementation uses `UserInfoResponse` struct that serializes to JSON via Serde.

## Goals / Non-Goals

**Goals:**
- Return human-readable text summary from GET `/api/users/{user_id}`
- Gracefully handle missing optional fields
- Maintain existing error handling behavior (404, 400, 503, 500)
- Simplify frontend by providing pre-formatted text

**Non-Goals:**
- Support both JSON and text formats (breaking change, text only)
- Internationalization or localization of format
- Configurable format strings
- Include password or sensitive fields in output

## Decisions

### Decision 1: Text Format Structure

**Choice**: `"Hello [Title] [Name], welcome! If we hear interesting news about [Hobby], we will let you know at [Email]!"`

**Rationale**:
- Natural reading order for a greeting message
- Shows context and interest of the hobby
- Exclamation marks show attitude

**Alternatives considered**:
- JSON with single `summary` field: Rejected to avoid mixed response types
- Structured format with labels: Rejected as too verbose for display purposes
- Debug-style string: Rejected as not user-friendly

### Decision 2: Name Construction Algorithm

**Choice**: Prioritized fallback logic
```
if first_name AND last_name → "FirstName LastName"
else if first_name → "FirstName"
else if last_name → "LastName"
else → username
```

**Rationale**:
- Full name is most professional when available
- Partial name better than falling back to username immediately
- Username as ultimate fallback always guaranteed to exist

**Alternatives considered**:
- Always show username: Rejected as redundant with real names
- Show username in addition to name: Rejected as cluttered

### Decision 3: Conditional Section Handling

**Choice**: Omit punctuation for missing optional fields

Examples:
- All fields: `"Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"`
- No hobby: `"Hello Software Engineer John Doe, welcome!"`
- No title: `"Hello John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"`
- No email: `"Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know!"`
- Minimal: `"Hello jdoe, welcome!"`

**Rationale**:
- Empty punctuation looks broken: `"John Doe () - , enjoys "`
- Clean omission reads naturally
- Edge case of only username still provides useful info

**Alternatives considered**:
- Placeholder text like "(no email)": Rejected as noisy
- Fixed structure with blank spaces: Rejected as awkward

### Decision 4: Error Response Format

**Choice**: Plain text error messages, not JSON

Examples:
- 404: `"User with ID 42 not found"`
- 400: `"user_id must be a valid integer"`
- 503: `"Database connection failed"`
- 500: `"Failed to fetch user"`

**Rationale**:
- Consistent response format (all text/plain)
- HTTP status code carries semantic meaning for client logic
- Frontend already shows "response text" per ui-user-info spec
- Simpler parsing (no JSON structure needed)

**Alternatives considered**:
- Keep errors as JSON: Rejected for format consistency
- HTTP status only: Rejected as frontend needs displayable message

### Decision 5: Response Content-Type

**Choice**: `Content-Type: text/plain; charset=utf-8`

**Rationale**:
- Explicitly declares non-JSON response
- UTF-8 handles international characters in names
- Standard MIME type for plain text

### Decision 6: Implementation Approach

**Choice**: Replace `UserInfoResponse` struct serialization with manual string formatting

Implementation location: Create a new function `get_user_info()` into a new rust file in the `src/rust/` folder.

**Rationale**:
- Serde JSON serialization no longer needed
- String formatting logic localized to handler
- Error responses also convert to plain text

**Alternatives considered**:
- Create new `impl Display for User`: Rejected as coupling business logic to data model
- Separate formatter service: Rejected as over-engineering for single endpoint

## Risks / Trade-offs

**Risk**: Breaking change for existing API clients
→ **Mitigation**: Clearly documented in proposal as BREAKING. Consider versioned endpoint (e.g., `/v2/users/{id}`) if compatibility needed in future. Since the change is still in development, the change is not breaking and no v2 is necessary.

**Risk**: Loss of structured data for frontend
→ **Mitigation**: UI spec already expects text display. If structured parsing needed later, this was the wrong design choice.

**Risk**: Format changes require code changes
→ **Mitigation**: Accepted trade-off. Format is intentionally fixed and simple. Business rules don't require frequent format updates.

**Risk**: Limited extensibility (e.g., adding phone number field)
→ **Mitigation**: Format intentionally minimal. New fields require design update. This prevents uncontrolled format bloat.

**Trade-off**: Less flexible than JSON
**Benefit**: Simpler frontend, single source of truth for formatting

**Trade-off**: Harder to parse programmatically
**Benefit**: Not a use case for this endpoint (display-only)
