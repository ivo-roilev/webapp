## Context

The existing web application is built with TypeScript, React, Vite, and React Router. While this architecture is production-grade, it introduces significant complexity for what are fundamentally three simple forms:
- Create user form
- Login form
- User info display

The result is 10,000+ build artifacts, complex build tooling, and steep learning curve for non-JavaScript developers. The application works well but is overkill for the user management feature.

The proposal seeks to drastically simplify the web layer while maintaining identical end-user functionality.

## Goals / Non-Goals

**Goals:**
- Reduce project complexity from 10,000+ files to ~5 files
- Eliminate build step - serve files directly without compilation
- Make code easy to understand and modify for non-JavaScript specialists
- Maintain identical UI/UX and functionality
- Use industry-standard HTML/CSS/JavaScript with no hidden abstractions

**Non-Goals:**
- Client-side state management library
- Routing library (use browser navigation instead)
- Component framework (use plain HTML instead)
- Build process or bundle optimization (serve static files)
- Progressive web app capabilities or offline support
- Advanced browser compatibility (assume modern browsers)

## Decisions

### Decision 1: Three Separate HTML Pages vs Single Route-Based Page
**Choice**: Three separate static HTML files (login.html, create-user.html, user-info.html)

**Rationale**:
- Eliminates need for client-side router
- Each page is self-contained and easy to understand
- Simple link-based navigation matches web fundamentals
- Reduces cognitive complexity

**Alternatives Considered**:
- Single index.html with JavaScript route handling - adds back complexity
- Server-side routing with template engine - requires backend changes

### Decision 2: Session Storage Strategy
**Choice**: Use `localStorage` for storing user_id after login/creation, with redirect to index/login on new session

**Rationale**:
- Browser localStorage persists across page reloads
- No server-side session management needed
- Logout simply clears localStorage
- Standard browser API, no library needed

**Alternatives Considered**:
- URL query parameters - exposes sensitive data in URL/history
- IndexedDB - overcomplicated for this simple use case

### Decision 3: Form Submission and API Communication
**Choice**: Vanilla JavaScript fetch API with inline event listeners

**Rationale**:
- Fetch is standard in all modern browsers
- Direct API calls without abstraction layers
- Inline JavaScript keeps code in one place per file
- No form library or validation framework needed - simple manual validation

**Alternatives Considered**:
- Axios library - adds dependency, no real benefit
- Form submit events with preventDefault - same result, slightly different pattern

### Decision 4: Styling
**Choice**: Single shared CSS file (style.css) linked from all pages

**Rationale**:
- Professional, responsive design in <500 lines of CSS
- Eliminates CSS-in-JS complexity
- Easy to maintain and override
- Browser caches stylesheet across pages

**Alternatives Considered**:
- Inline styles in each HTML - difficult to maintain consistency
- CSS frameworks like Bootstrap - adds unnecessary bloat

### Decision 5: Error Handling and Validation
**Choice**: Simple client-side validation with inline error messages

**Rationale**:
- Clear, immediate feedback to users
- Required fields (username, password) validated before API calls
- API errors displayed as user-friendly messages
- No form validation library needed

### Decision 6: Landing/Index Page Strategy
**Choice**: login.html as the default/index page, with button navigation

**Rationale**:
- Login is the primary user action
- Reduces unnecessary navigation steps
- Create user link available on login page
- Matches typical web app flow

## Risks / Trade-offs

| Risk | Impact | Mitigation |
|------|--------|-----------|
| No build optimization | Slightly larger JavaScript in production | Minimal - each file is small (<2KB JS). Browser caching handles it. |
| Loss of TypeScript type checking | Potential runtime errors in JavaScript | Simple, linear code reduces errors. Manual type discipline in JSDoc comments possible later. |
| No tree-shaking or dependency management | Cannot optimize for unused code | No dependencies = no bloat. Code is already minimal. |
| Three page loads instead of SPA | Slightly more network round-trips | Acceptable. Modern browsers cache everything. Initial load is faster anyway. |
| No advanced browser feature detection | May break in very old browsers | Assume modern browser support (Chrome 90+, Safari 14+, Firefox 88+). Clearly document this. |

## Migration Plan

### Phase 1: Parallel Implementation
- No need to keep the existing React app running in production - the feature is still in development
- Build new static HTML pages in src/web/ directory replacing all of the existing files
- Thoroughly clean the previous implementation - the history is kept in git

### Phase 2: Test New Static Pages
- Run static pages locally on port 3000 or similar
- Test against same API endpoints as React app (running on port 8080)
- Verify all three flows: login, create user, view user info

### Phase 3: Cutover
- Update web server to serve from src/web/ directory
- Verify logs show all requests hitting correct endpoints
- Test in staging environment with real API

### Phase 4: Cleanup
- Remove unused React files (package.json, tsconfig, node_modules, dist/, src/)
- Remove Vite configuration
- Re-generate the src/web/ HTML/CSS files

### Rollback Strategy
- Still in development, no need for a rollback. Git keeps the history anyway
- No database changes needed (same API endpoints)

## Open Questions

1. Should we support older browsers (IE, Safari <14)? → Recommend NO, document minimum versions
3. Should we add HTTPS/security headers? → Yes, but at web server level, not in HTML/JS
4. Should we add analytics or tracking? → Recommend NO for MVP, add later if needed
5. How should the environment (API base URL) be configured? → Currently hardcoded to `http://localhost:8080`. Should this use `.env` or config file?: Use the recommended server configuration method.
