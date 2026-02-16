# Implementation Tasks

## 1. Backend Setup - Add Dependency

- [ ] 1.1 Open `Cargo.toml` in project root
- [ ] 1.2 Add `actix-files = "0.6"` to `[dependencies]` section
- [ ] 1.3 Run `cargo check` to verify dependency resolves
- [ ] 1.4 Commit Cargo.toml and Cargo.lock changes

## 2. Backend Implementation - Configure Static File Serving

- [ ] 2.1 Open `src/rust/main.rs`
- [ ] 2.2 Add import: `use actix_files as fs;` at top of file
- [ ] 2.3 Locate the `.service()` calls for API routes
- [ ] 2.4 Add static file service AFTER all API routes:
  ```rust
  .service(
      fs::Files::new("/", "./src/web")
          .index_file("index.html")
          .use_last_modified(true)
  )
  ```
- [ ] 2.5 Verify route order: health check → API routes → static files
- [ ] 2.6 Save file

## 3. Backend Testing - Verify Static Files Served

- [ ] 3.1 Run `cargo run` from project root
- [ ] 3.2 Open browser to `http://localhost:8080/`
- [ ] 3.3 Verify index.html loads (login page displays)
- [ ] 3.4 Check browser DevTools Network tab for 200 OK status
- [ ] 3.5 Navigate to `http://localhost:8080/create-user.html`
- [ ] 3.6 Verify create-user page loads
- [ ] 3.7 Navigate to `http://localhost:8080/style.css`
- [ ] 3.8 Verify CSS file loads with correct Content-Type
- [ ] 3.9 Test 404: Navigate to `http://localhost:8080/nonexistent.html`
- [ ] 3.10 Verify 404 Not Found response

## 4. Frontend Updates - Login Page (index.html)

- [ ] 4.1 Open `src/web/index.html`
- [ ] 4.2 Locate the `<form>` element
- [ ] 4.3 Find `action` attribute (currently has full URL)
- [ ] 4.4 Change `action="http://localhost:8080/api/login"` to `action="/api/login"`
- [ ] 4.5 Search for any other localhost references in the file
- [ ] 4.6 Save file

## 5. Frontend Updates - Create User Page

- [ ] 5.1 Open `src/web/create-user.html`
- [ ] 5.2 Locate the `<form>` element
- [ ] 5.3 Change `action="http://localhost:8080/api/create-user"` to `action="/api/create-user"`
- [ ] 5.4 Locate link back to login/index page
- [ ] 5.5 Change any `href="http://localhost:8080/index.html"` to `href="/"` or `href="index.html"`
- [ ] 5.6 Search for any other localhost references
- [ ] 5.7 Save file

## 6. Frontend Updates - User Info Page

- [ ] 6.1 Open `src/web/user-info.html`
- [ ] 6.2 Locate the JavaScript `<script>` section
- [ ] 6.3 Find the `fetch()` call (fetches user info from API)
- [ ] 6.4 Change fetch URL from `http://localhost:8080/api/users/${userId}` to `/api/users/${userId}`
- [ ] 6.5 Locate links back to login or create-user pages
- [ ] 6.6 Update any absolute URLs to relative URLs
- [ ] 6.7 Search for any other localhost references
- [ ] 6.8 Save file

## 7. Frontend Testing - Forms with Relative URLs

- [ ] 7.1 Restart server: `cargo run` (if not already running)
- [ ] 7.2 Open `http://localhost:8080/` in browser
- [ ] 7.3 Fill in login form with test credentials
- [ ] 7.4 Submit form
- [ ] 7.5 Verify form submits to `/api/login` (check Network tab)
- [ ] 7.6 Verify redirect or response works correctly
- [ ] 7.7 Navigate to create-user page
- [ ] 7.8 Fill in create user form
- [ ] 7.9 Submit form
- [ ] 7.10 Verify form submits to `/api/create-user`
- [ ] 7.11 Verify user-info page loads with data
- [ ] 7.12 Verify fetch call uses `/api/users/{id}` (check Network tab)

## 8. Documentation Updates - Web README

- [ ] 8.1 Open `src/web/README.md`
- [ ] 8.2 Locate "Local Development" or similar section
- [ ] 8.3 Remove instructions for running Python web server
- [ ] 8.4 Remove any mentions of `python3 -m http.server`
- [ ] 8.5 Update to show single command: `cargo run`
- [ ] 8.6 Add note that server serves both API and static files
- [ ] 8.7 Add note about running from project root directory
- [ ] 8.8 Save file

## 9. Documentation Updates - Project README

- [ ] 9.1 Open main `README.md` in project root
- [ ] 9.2 Search for any mentions of Python web server
- [ ] 9.3 Remove or update Python server instructions
- [ ] 9.4 Verify "Getting Started" section mentions only `cargo run`
- [ ] 9.5 Update dependencies section (remove Python requirement if listed)
- [ ] 9.6 Save file (if changes needed)

## 10. Cleanup - Remove Python Server References

- [ ] 10.1 Search entire project for "python3 -m http.server"
- [ ] 10.2 Search for "8080" in documentation files
- [ ] 10.3 Update or remove any scripts that start Python server
- [ ] 10.4 Check for any shell scripts (.sh files) that reference Python
- [ ] 10.5 Update or remove those references

## 11. Integration Testing - Full Workflow

- [ ] 11.1 Stop any running servers
- [ ] 11.2 Run `cargo run` from project root
- [ ] 11.3 Verify server starts successfully
- [ ] 11.4 Verify log message shows correct bind address (e.g., "listening on 127.0.0.1:8080")
- [ ] 11.5 Test health endpoint: `curl http://localhost:8080/health`
- [ ] 11.6 Verify 200 OK response
- [ ] 11.7 Test static file: `curl http://localhost:8080/`
- [ ] 11.8 Verify HTML content returned
- [ ] 11.9 Test API endpoint: `curl -X POST http://localhost:8080/api/login -d "username=test&password=test"`
- [ ] 11.10 Verify API responds correctly

## 12. Integration Testing - Browser End-to-End

- [ ] 12.1 Open browser to `http://localhost:8080/`
- [ ] 12.2 Verify CSS loads and page looks correct
- [ ] 12.3 Create a new user via the create-user form
- [ ] 12.4 Verify redirect to user-info page
- [ ] 12.5 Verify user info displays correctly
- [ ] 12.6 Navigate back to login
- [ ] 12.7 Log in with created user
- [ ] 12.8 Verify redirect to user-info page
- [ ] 12.9 Verify greeting message displays

## 13. Existing Tests - Verify No Regressions

- [ ] 13.1 Run unit tests: `cargo test`
- [ ] 13.2 Verify all existing tests pass
- [ ] 13.3 If any tests fail, investigate and fix
- [ ] 13.4 Check if integration tests exist
- [ ] 13.5 Run integration tests if present
- [ ] 13.6 Verify no test failures

## 14. Edge Case Testing

- [ ] 14.1 Test with different working directory:
  - [ ] 14.1.1 `cd src/rust && cargo run`
  - [ ] 14.1.2 Verify static files still load (or document that project root is required)
- [ ] 14.2 Test API route precedence:
  - [ ] 14.2.1 Verify `/api/login` hits API, not static file
  - [ ] 14.2.2 Verify `/api/create-user` hits API
  - [ ] 14.2.3 Verify `/health` hits health endpoint
- [ ] 14.3 Test missing files:
  - [ ] 14.3.1 Request nonexistent file
  - [ ] 14.3.2 Verify 404 response
- [ ] 14.4 Test CSS caching:
  - [ ] 14.4.1 Load page, check Last-Modified header
  - [ ] 14.4.2 Reload page
  - [ ] 14.4.3 Verify 304 Not Modified if appropriate

## 15. Performance Verification

- [ ] 15.1 Use browser DevTools Performance tab
- [ ] 15.2 Record page load for index.html
- [ ] 15.3 Verify static files load quickly
- [ ] 15.4 Check for any slow requests
- [ ] 15.5 Verify caching headers present (Last-Modified)
- [ ] 15.6 Reload and verify 304 responses for cached files

## 16. Documentation - Code Comments

- [ ] 16.1 Add comment above static file service explaining purpose
- [ ] 16.2 Add comment about route order importance
- [ ] 16.3 Add comment about working directory requirement
- [ ] 16.4 Review all code changes for clarity

## 17. Final Review and Commit

- [ ] 17.1 Review all changed files
- [ ] 17.2 Verify no debug code left behind
- [ ] 17.3 Verify no commented-out code
- [ ] 17.4 Run `cargo fmt` to format Rust code
- [ ] 17.5 Run `cargo clippy` to check for issues
- [ ] 17.6 Stage all changes: `git add .`
- [ ] 17.7 Commit with descriptive message: `git commit -m "Serve static files from Rust backend, eliminate Python server dependency"`
- [ ] 17.8 Push changes: `git push`

## Success Criteria Checklist

- [ ] ✅ Single `cargo run` starts both API and static serving
- [ ] ✅ No Python web server needed
- [ ] ✅ Static files load from `http://localhost:8080/`
- [ ] ✅ API endpoints work at `http://localhost:8080/api/*`
- [ ] ✅ HTML forms use relative URLs
- [ ] ✅ All existing functionality preserved
- [ ] ✅ All tests pass
- [ ] ✅ Documentation updated
- [ ] ✅ No localhost hardcoded in HTML files