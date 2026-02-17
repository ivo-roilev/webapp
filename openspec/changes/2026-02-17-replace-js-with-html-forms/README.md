# Replace JavaScript Form Handling with HTML Forms and Server Redirects

## Overview

This OpenSpec change proposes simplifying the web application by removing client-side JavaScript form handling and replacing it with traditional HTML form submissions with server-side redirects using the POST-Redirect-GET pattern.

## Problem Statement

Currently, the application uses JavaScript to:
- Intercept form submissions
- Make AJAX/fetch requests to API endpoints
- Handle client-side redirects and error displays

This approach requires JavaScript to be enabled and adds complexity to the frontend.

## Proposed Solution

Replace JavaScript form handling with:
- Direct HTML form submissions to API endpoints
- Server-side HTTP 303 redirects (POST-Redirect-GET pattern)
- Minimal JavaScript only for displaying error messages from query parameters

## Key Benefits

1. **Simpler Code**: Remove ~100 lines of JavaScript
2. **Accessibility**: Works without JavaScript enabled
3. **Progressive Enhancement**: Basic functionality without JS, enhanced with JS
4. **Standard Web Patterns**: Follows HTTP redirect patterns
5. **Better Performance**: Faster initial page load

## Documentation Structure

This change follows the OpenSpec spec-driven workflow:

### 📄 [proposal.md](./proposal.md)
High-level overview of the change:
- Current state and issues
- Proposed changes and benefits
- Trade-offs and recommendations
- Success criteria

### 📐 [design.md](./design.md)
Detailed technical design:
- Architecture diagrams
- Component design (backend and frontend)
- Error handling strategy
- Testing approach
- Security and performance considerations

### ✅ [tasks.md](./tasks.md)
Implementation tasks breakdown:
- Phase-by-phase task lists
- Acceptance criteria for each task
- Time estimates
- Testing checklist

### 📋 specs/
Detailed specifications for components:

#### [backend-redirects.md](./specs/backend-redirects.md)
- Login endpoint modifications
- Create user endpoint modifications
- Error code mapping
- Test cases

#### [frontend-forms.md](./specs/frontend-forms.md)
- HTML changes for login page
- HTML changes for create user page
- Error message display
- Progressive enhancement strategy

## Implementation Status

**Status**: 📝 Proposal Phase (Not Yet Implemented)

This is a proposal document. To implement this change:

1. Review and approve the proposal
2. Follow the tasks in `tasks.md`
3. Implement backend changes first (endpoints return redirects)
4. Update frontend HTML files
5. Update tests
6. Validate complete user flows

## Quick Start for Implementation

If you want to implement this change:

```bash
# 1. Read the proposal
cat proposal.md

# 2. Review the design
cat design.md

# 3. Follow the tasks
cat tasks.md

# 4. Implement backend changes (Phase 1)
# Edit src/rust/main.rs following specs/backend-redirects.md

# 5. Update frontend (Phase 2)
# Edit HTML files following specs/frontend-forms.md

# 6. Test (Phase 3)
cargo test
# Manual testing with browser
```

## Key Changes Summary

### Backend (`src/rust/main.rs`)

**Before:**
```rust
// Success
HttpResponse::Ok().body(user_id.to_string())

// Error
HttpResponse::Unauthorized().json(ErrorResponse { ... })
```

**After:**
```rust
// Success
HttpResponse::SeeOther()
    .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
    .finish()

// Error
HttpResponse::SeeOther()
    .append_header(("Location", "/index.html?error=invalid_credentials"))
    .finish()
```

### Frontend (`src/web/*.html`)

**Before:**
```html
<script>
  // 50+ lines of JavaScript for form handling
  document.getElementById('form').addEventListener('submit', async (e) => {
    e.preventDefault();
    // fetch, error handling, redirect...
  });
</script>
```

**After:**
```html
<!-- Form submits directly, no preventDefault -->
<form method="POST" action="/api/login">
  <!-- fields -->
</form>

<!-- Minimal JS only for error display -->
<script>
  // 10 lines to show error from query param
  const error = new URLSearchParams(window.location.search).get('error');
  if (error) {
    showError(error);
  }
</script>
```

## User Flow Comparison

### Current Flow (JavaScript)
```
User submits → JS prevents default → JS makes fetch → JS handles response → JS redirects
```

### Proposed Flow (HTML Forms)
```
User submits → Browser POSTs → Server redirects → Browser follows redirect
```

## Estimated Effort

**Total Time**: ~4-5 hours

- Backend changes: 1 hour
- Frontend changes: 1 hour
- Testing: 1.5 hours
- Documentation: 1 hour

## Questions or Feedback?

For questions about this proposal:
1. Review the detailed design document
2. Check the task breakdown
3. See specific implementation specs

## Related Changes

- **2026-02-16-serve-static-files-from-rust**: Consolidated static file serving into Rust backend (prerequisite for this change)

## References

- [POST-Redirect-GET Pattern](https://en.wikipedia.org/wiki/Post/Redirect/Get)
- [HTTP 303 See Other](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303)
- [Progressive Enhancement](https://developer.mozilla.org/en-US/docs/Glossary/Progressive_Enhancement)
