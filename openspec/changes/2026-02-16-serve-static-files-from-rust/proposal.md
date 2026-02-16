# Proposal: Serve Static HTML Files from Rust Backend

## Overview

Eliminate the need for a separate Python web server by serving static HTML/CSS files directly from the Actix-web Rust backend. This simplifies the development and deployment workflow by consolidating both API endpoints and static file serving into a single server process.

## Current State

The application currently requires **two separate web servers** running simultaneously:

- **Python HTTP Server** (port 8080): Serves static HTML files from `src/web/` directory
  - `python3 -m http.server 8080` in `src/web/` directory
  - Serves `index.html`, `create-user.html`, `user-info.html`, `style.css`

- **Rust Actix-web Server** (port 8080): Serves REST API endpoints
  - `cargo run` from project root
  - Handles `/api/login`, `/api/create-user`, `/api/users/{id}`, `/health`

### Current Issues

1. **Port Conflict**: Both servers want port 8080, requiring manual coordination
2. **Two Processes**: Developers must run two separate commands in different terminals
3. **CORS Complexity**: Separate origins may require CORS headers (currently using permissive CORS)
4. **Deployment Complexity**: Need to deploy and manage two separate services
5. **HTML Form URLs**: Forms use absolute URLs like `http://localhost:8080/api/login` instead of relative paths

## Proposed Changes

### Backend Changes

**1. Add `actix-files` dependency to Cargo.toml:**
```toml
[dependencies]
actix-files = "0.6"
```

**2. Configure static file serving in main.rs:**
- Import `actix_files` module
- Add static file service after API routes
- Serve files from `./src/web` directory
- Set `index.html` as default file for root path
- Enable caching headers for performance

**Route Priority:**
1. `/health` → Health check endpoint (most specific)
2. `/api/*` → API endpoints (create-user, login, users)
3. `/*` → Static files from `src/web/` (catch-all)

### Frontend Changes

**1. Update HTML forms to use relative URLs:**
- Change `action="http://localhost:8080/api/login"` → `action="/api/login"`
- Change `action="http://localhost:8080/api/create-user"` → `action="/api/create-user"`
- Update JavaScript fetch calls to use relative URLs

**Files to update:**
- `src/web/index.html` (login form)
- `src/web/create-user.html` (create user form)
- `src/web/user-info.html` (fetch call for user info)

### Documentation Changes

**1. Update README files:**
- Remove Python web server instructions
- Simplify local development section to single `cargo run` command
- Update "Local Development" section in `src/web/README.md`
- Update main project README if it mentions web server setup

## Benefits

1. **Single Command**: Start entire application with just `cargo run`
2. **No Port Conflicts**: One server on one port
3. **Simpler CORS**: Same-origin requests eliminate CORS complexity
4. **Production Ready**: Actix-web efficiently serves static files with compression and caching
5. **Single Binary**: Can embed static files in compiled binary for deployment (future enhancement)
6. **Better DX**: Developers don't need Python installed
7. **Consistent URLs**: Relative URLs work on any port/domain

## Trade-offs

1. **Hot Reload**: Static HTML changes require no rebuild (Actix-web serves files directly from disk)
2. **Rust Changes**: Still require `cargo run` restart for backend changes (unchanged)
3. **File Path Dependency**: Binary must be run from project root or configure static file path

## Out of Scope

- Embedding static files into the binary (using `rust-embed` or `include_dir`)
- Asset compression or minification
- Cache-busting for CSS/JS files
- CDN integration
- Multiple static file directories

## Success Criteria

- ✅ Single `cargo run` command starts both API and static file serving
- ✅ Static files accessible at `http://localhost:8080/`
- ✅ API endpoints continue to work at `http://localhost:8080/api/*`
- ✅ HTML forms use relative URLs and work correctly
- ✅ No Python web server needed for development
- ✅ All existing tests continue to pass
- ✅ Documentation updated to reflect single-server setup

## Timeline

- **Backend Implementation**: 15 minutes (add dependency, update main.rs)
- **Frontend Updates**: 10 minutes (update form action URLs)
- **Documentation Updates**: 10 minutes (update READMEs)
- **Testing & Validation**: 15 minutes (verify all routes work)

**Total Estimated Effort**: ~1 hour

## Additional Notes

This change aligns with standard web application architecture where a single backend server handles both API endpoints and static asset serving. The Actix-web framework is production-ready and efficient for this use case, with built-in support for caching, compression, and range requests.

Future enhancements could include embedding static files directly into the compiled binary for true single-file deployment, but serving from the filesystem is simpler for development and allows HTML/CSS changes without recompilation.
