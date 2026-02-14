# Design: Minimal JavaScript Web UI with Native HTML Forms

## Context

The current web UI follows a SPA-style architecture with JSON REST APIs, despite being a simple form-based application. This mismatch has resulted in ~325 lines of JavaScript that duplicate functionality already handled by browsers and the backend:

**Current Implementation:**
- Three HTML pages (login, create-user, user-info) each containing ~30 lines of theme toggle JS
- Forms use `preventDefault()` + `fetch()` to submit via JSON API (~150 lines)
- Client-side validation duplicating backend logic (~85 lines)
- localStorage for session-like state (user_id) without actual security
- Backend returns JSON requiring JavaScript to parse and handle redirects

**Key Pain Points:**
1. Theme toggle code is duplicated across all three pages
2. Simple forms require complex JavaScript to interact with JSON APIs
3. Client-side validation adds complexity without providing real value (backend validates anyway)
4. No real progressive enhancement - pages require JS for basic functionality

**Stakeholders:**
- Learning/experimentation context - trade-offs acceptable for understanding HTML-first patterns
- No production users affected

**Constraints:**
- Keep existing backend structure (Actix-web, Rust handlers)
- Maintain all current functionality (login, create user, view info)
- Preserve existing test suite
- Use static HTML files (no server-side rendering/templates)

## Goals / Non-Goals

**Goals:**
1. **Reduce JavaScript from ~325 lines to ~15 lines** - Remove all unnecessary code
2. **Use native browser capabilities** - Forms, redirects, CSS media queries
3. **Simplify architecture** - Traditional form POST → backend redirect pattern
4. **Maintain functionality** - All current features work exactly as before
5. **Improve maintainability** - Less code to debug and test
6. **Enable progressive enhancement** - Most features work without JS

**Non-Goals:**
- Session management or authentication tokens (not in current design)
- Server-side rendering with HTML templates (keeping static files)
- Complete zero-JavaScript (user-info display needs ~15 lines for fetch)
- Progressive Web App features
- Supporting clients without JavaScript entirely
- Production-grade security patterns

## Decisions

### Decision 1: Backend Returns Plain Text User ID, Frontend Handles Redirect

**Choice:** POST endpoints return `200 OK` with plain text user_id. Minimal JavaScript (~12 lines per form) intercepts submission, sends form data via fetch(), and redirects client-side.

**Rationale:**
- Cleaner API separation: backend returns data, frontend handles navigation
- Plain text response simpler than JSON (no parsing overhead)
- Minimal JavaScript (~12 lines) still achieves 96% reduction from original ~325 lines
- Frontend has full control over redirect behavior and error handling
- Backend endpoints remain RESTful and easy to test

**Alternatives Considered:**
- **HTTP 303 Redirects**: Browser follows automatically but backend couples data response with navigation logic; forms submit natively but error handling is harder
- **Keep JSON API**: Doesn't achieve simplification goal; requires more complex response parsing
- **Return HTML directly from POST**: Less RESTful, harder to test, breaks API clients

**Implementation:**
Backend handlers return HTTP 200 OK with content type "text/plain" and the user_id as the response body (e.g., "1"). Frontend JavaScript intercepts form submission, converts FormData to URLSearchParams for proper form encoding, sends the POST request with appropriate headers, reads the plain text response, and performs client-side navigation to the user-info page with the user_id as a query parameter.

### Decision 2: Remove Theme Toggle, Use CSS Media Queries Only

**Choice:** Delete all theme toggle JavaScript, rely on `@media (prefers-color-scheme)` CSS queries.

**Rationale:**
- Theme toggle is ~30% of total JavaScript (90 lines duplicated across 3 files)
- System preference is becoming standard (respects user's OS-wide choice)
- Zero JavaScript required for theme support
- Simpler maintenance - no localStorage, no event handlers, no duplicated code

**Alternatives Considered:**
- **Keep toggle, deduplicate code**: Still ~30 lines JS, requires localStorage, adds complexity
- **Use CSS-only toggle (hidden checkbox)**: Clever but hacky, doesn't persist, poor UX
- **Remove dark theme entirely**: Loses accessibility feature

**Implementation:**
Style.css defines separate CSS media queries for light and dark color schemes using the prefers-color-scheme media feature. Each media query sets CSS custom properties for colors (background, text, borders, etc.) appropriate to that theme. The browser automatically applies the correct theme based on the user's system preference without any JavaScript.

### Decision 3: Replace localStorage with URL Query Parameters

**Choice:** Pass user_id via URL params (`?user_id=123`) instead of localStorage.

**Rationale:**
- localStorage was used for session-like behavior without actual security
- URL params are visible but we don't have auth tokens or sensitive data
- Simpler implementation - no storage API calls, no checking if stored value exists
- Works with browser back/forward, can bookmark specific user page
- Backend redirect naturally constructs URL with params

**Alternatives Considered:**
- **Use HTTP-only cookies**: More secure but requires backend session management (out of scope)
- **Keep localStorage**: Doesn't work with native form submission
- **Server-side sessions**: Requires session store, cookies, significantly more complex

**Security Note:**
- User ID in URL is visible, but this is acceptable for learning context
- No password or sensitive data exposed
- Same user ID already returned in JSON response previously
- Browser doesn't follow redirect for POST in cross-origin scenarios

### Decision 4: Remove Client-Side Validation, Keep HTML5 Only

**Choice:** Delete JavaScript validation logic, rely on HTML5 `required` attributes and backend validation.

**Rationale:**
- Backend already validates all fields (length, format, required)
- Database has constraints (username uniqueness, field lengths)
- Client-side validation is redundant and adds ~85 lines of JavaScript
- HTML5 `required` provides basic UX feedback without JavaScript
- Form submission is fast enough that immediate JS validation isn't critical

**Alternatives Considered:**
- **Keep minimal JS validation**: Still adds complexity, not worth 85 lines
- **Remove HTML5 attributes too**: Loses basic browser UX for free

**Validation Flow:**
1. HTML5 `required` prevents empty submission (browser UI)
2. Backend validates input (length, format, business rules)
3. Backend returns appropriate HTTP status (400/409/500)
4. Browser shows error page or backend can return custom error HTML

### Decision 5: Keep Minimal JavaScript for Form Submission and Display

**Choice:** ~12 lines of JavaScript in login/create-user pages for form submission, and ~15 lines in user-info.html to fetch and display user greeting.

**Rationale:**
- Form submission JS enables better error handling and smooth UX
- Backend returns data (user_id), frontend handles navigation
- Acceptable trade-off: 96% reduction (325→~40 total lines) while maintaining smooth UX
- GET endpoint already returns text, minimal change needed for display
- JavaScript is focused and purposeful (submission + display only)

**Alternatives Considered:**
- **Zero JavaScript with native form submission + 303 redirects**: Error handling is harder, less control over navigation
- **Server-side rendering**: Out of scope, requires template engine
- **Inline data in redirect**: Would need to return HTML from POST

**Implementation:**
Form pages add event listeners to intercept submission, prevent default behavior, collect form data, convert to URLSearchParams, POST to the backend with form-encoded content type, read the plain text user_id response, and navigate to the user-info page. The user-info page extracts user_id from URL query parameters using URLSearchParams, redirects to index.html if missing, fetches the greeting text from the backend API, and displays it in the DOM with basic error handling.

### Decision 6: Rename POST /api/users to POST /api/create-user

**Choice:** Change endpoint path from `/api/users` to `/api/create-user`.

**Rationale:**
- Avoids confusion with `GET /api/users/{user_id}` endpoint
- Makes the endpoint purpose more explicit
- More descriptive for form action URLs
- Better aligns with RESTful convention (POST to collection can be ambiguous)

**Alternatives Considered:**
- **Keep /api/users**: Following RESTful convention but confusing with GET endpoint
- **Use /api/users/new**: More RESTful but longer action URL

**Implementation:**
Backend route configuration registers the endpoint at /api/create-user instead of /api/users. HTML forms update their action attribute to point to the new endpoint path.

### Decision 7: Remove Redundant Page Headings

**Choice:** Remove `<h1>User Information</h1>` from user-info.html.

**Rationale:**
- The heading provides no value to users viewing the page
- The user's personalized greeting serves as the main content
- Page title in browser tab already identifies the page
- Simpler, cleaner UI with less visual clutter
- Consistent with minimalist design approach of this change

**Alternatives Considered:**
- **Keep heading**: Adds no value, takes up space
- **Replace with dynamic heading**: Unnecessary when greeting is the main content
- **Remove from all pages**: Other pages may benefit from headings, evaluate case-by-case

**Implementation:**
The H1 heading element containing "User Information" is removed from user-info.html, leaving only the greeting container and its content within the card structure.

### Decision 8: Rename login.html to index.html

**Choice:** Rename login.html to index.html to serve as the application entry point.

**Rationale:**
- Eliminates the redirect-only index.html file
- Makes login page the natural entry point when browsing to root
- Simpler file structure - one less file to maintain
- No JavaScript redirect needed for entry point
- Standard web convention (index.html as default page)

**Alternatives Considered:**
- **Keep separate index.html**: Unnecessary extra file and redirect
- **Use meta refresh in index.html**: Still an extra file and redirect layer
- **Keep login.html name**: Would require explicit URL or redirect from index

**Implementation:**
The login.html file is renamed to index.html. All references to "login.html" in other HTML files (create-user.html and user-info.html) are updated to point to "index.html" instead.

## Risks / Trade-offs

### Risk: User ID Visible in URL
**Impact:** User can see their numeric ID in the URL bar.

**Mitigation:**
- Acceptable for learning/experimentation context
- No sensitive data exposed (username/password not in URL)
- Backend still validates user_id on GET request
- If needed later, can add session cookies without changing forms

### Risk: No Dark Mode Toggle
**Impact:** Users cannot switch theme without changing OS preference.

**Mitigation:**
- System preference is increasingly standard behavior
- Respects user's global choice across all applications
- If needed later, can add back with ~10 lines JS and localStorage
- Most users set OS theme and don't toggle frequently

### Risk: Less Immediate Validation Feedback
**Impact:** Form validation errors only shown after backend response.

**Mitigation:**
- HTML5 `required` still provides immediate feedback for empty fields
- Modern browsers validate quickly, round-trip is acceptable
- Backend validation is definitive anyway (client validation can be bypassed)
- Can add targeted JS validation for specific fields if needed

### Risk: Error Handling Without JavaScript
**Impact:** Backend errors shown as plain text or default browser error pages.

**Mitigation:**
- Backend can return simple HTML error pages with status codes
- Browser shows meaningful messages for common errors (404, 500)
- User can use browser back button to return to form
- Can enhance with custom error pages if needed

### Risk: Breaking Existing Tests
**Impact:** Handler tests expect JSON responses, will fail after changes to plain text.

**Mitigation:**
- Update test assertions to expect 200 OK status and plain text content type
- Test response body contains valid user_id integer
- Update test fixtures to send form-encoded data instead of JSON
- Estimated ~30min to update test suite

### Risk: Minimal JavaScript Still Required
**Impact:** Forms require JavaScript to function, no progressive enhancement to zero-JS.

**Mitigation:**
- Achieved 96% reduction in JavaScript (325→40 lines)
- JavaScript is focused and minimal (~12 lines per form)
- Could implement fallback to native form submission with meta refresh if needed
- Trade-off accepted for better error handling and UX

## Migration Plan

**Prerequisites:**
- Backup current web/ directory
- Ensure all existing tests pass
- Review proposal with stakeholders

**Implementation Order:**

1. **Update Backend (30 minutes)**
   - Rename route `/api/users` → `/api/create-user`
   - Modify login handler to return 303 redirect
   - Modify create_user handler to return 303 redirect
   - Update handler tests to expect redirects
   - Run tests: `cargo test`

2. **Update Frontend (45 minutes)**
   - Update style.css with @media queries
   - Remove theme toggle JS from all HTML files
   - Update form actions to point to correct endpoints
   - Remove form submit event handlers
   - Remove client-side validation JavaScript
   - Update user-info.html with minimal fetch code
   - Remove localStorage usage
   - Remove redundant `<h1>` heading from user-info.html
   - Delete or rename index.html

3. **Manual Testing (30 minutes)**
   - Test login flow end-to-end
   - Test create user flow end-to-end
   - Test user-info display
   - Verify theme switching with OS preference
   - Test error cases (invalid login, duplicate username)
   - Verify browser back/forward behavior

**Rollback Strategy:**
- Git branch for this change: easy to revert
- Static files can be replaced instantly
- Backend handlers easily reverted (one file)
- No database changes required
- Zero-downtime rollback possible

**Deployment:**
- Static files: replace in-place or deploy to CDN
- Backend: standard cargo build + restart
- No configuration changes needed
- No database migrations

## Open Questions

None - design is straightforward and well-understood. Implementation can proceed.
