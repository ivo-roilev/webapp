## 1. Backend Dependency Setup

- [ ] 1.1 Add `actix-files = "0.6"` to Cargo.toml dependencies
- [ ] 1.2 Verify Cargo.toml compiles with new dependency
- [ ] 1.3 Run `cargo update` to update Cargo.lock

## 2. Static File Serving Implementation

- [ ] 2.1 Import `actix_files::Files` in main.rs
- [ ] 2.2 Configure static file service for `./src/web` directory
- [ ] 2.3 Set `index.html` as default file for root path `/`
- [ ] 2.4 Configure static file service with proper ordering (after API routes)
- [ ] 2.5 Add route priority configuration: `/health` → `/api/*` → `/*` (static files)
- [ ] 2.6 Enable caching headers for static files
- [ ] 2.7 Test static file serving with `cargo run`

## 3. Frontend URL Updates

- [ ] 3.1 Update `src/web/index.html` form action from absolute to relative URL `/api/login`
- [ ] 3.2 Update `src/web/create-user.html` form action to relative URL `/api/create-user`
- [ ] 3.3 Update JavaScript fetch call in `src/web/user-info.html` to use relative URL `/api/users/{id}`
- [ ] 3.4 Remove all `http://localhost:8080` hardcoded URLs from HTML files
- [ ] 3.5 Verify all form submissions work with relative URLs

## 4. Documentation Updates

- [ ] 4.1 Update `src/web/README.md` to remove Python web server instructions
- [ ] 4.2 Update `src/web/README.md` with new single-command workflow (`cargo run`)
- [ ] 4.3 Update main project README.md if it references Python web server
- [ ] 4.4 Add note about static file serving in Rust service README
- [ ] 4.5 Update local development instructions to single `cargo run` command

## 5. Testing & Validation

- [ ] 5.1 Test starting server with `cargo run` (should serve both API and static files)
- [ ] 5.2 Verify `http://localhost:8080/` loads `index.html` correctly
- [ ] 5.3 Verify `http://localhost:8080/index.html` loads correctly
- [ ] 5.4 Verify `http://localhost:8080/create-user.html` loads correctly
- [ ] 5.5 Verify `http://localhost:8080/user-info.html` loads correctly
- [ ] 5.6 Verify `http://localhost:8080/style.css` loads correctly
- [ ] 5.7 Test login form submission with relative URL
- [ ] 5.8 Test create user form submission with relative URL
- [ ] 5.9 Test user info page fetch with relative URL
- [ ] 5.10 Verify no CORS issues with same-origin requests
- [ ] 5.11 Verify health check endpoint `/health` still works
- [ ] 5.12 Verify all API endpoints still work (`/api/login`, `/api/create-user`, `/api/users/{id}`)

## 6. Cleanup & Finalization

- [ ] 6.1 Remove Python web server startup instructions from docs
- [ ] 6.2 Verify no references to `python3 -m http.server` remain in docs
- [ ] 6.3 Test full user flow: login → create user → view user info
- [ ] 6.4 Verify port 8080 is only used by one process (Rust server)
- [ ] 6.5 Run existing Rust tests to ensure no regressions
- [ ] 6.6 Update any CI/CD configurations if they reference Python web server

## Success Criteria

- ✅ Single `cargo run` command starts both API and static file server
- ✅ No Python web server required
- ✅ All HTML pages load correctly from Rust server
- ✅ All form submissions work with relative URLs
- ✅ No CORS errors in browser console
- ✅ All API endpoints continue to work
- ✅ Health check endpoint remains accessible
- ✅ Documentation updated to reflect single-server setup
