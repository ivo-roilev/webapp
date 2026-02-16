## Context

The application currently requires running two separate web servers: a Python HTTP server for static files and the Rust Actix-web server for API endpoints. Both servers attempt to bind to port 8080, creating configuration complexity and requiring developers to manage two processes during development. This design consolidates both functions into the Actix-web server.

**Current Architecture:**
- Python server (`python3 -m http.server 8080`) → serves HTML/CSS from `src/web/`
- Rust server (`cargo run`) → serves API endpoints at `/api/*`
- Problems: Port conflict, two terminal sessions, CORS configuration, deployment complexity

**Stakeholders:** Developers (simplified workflow), Operations (single deployment unit)

**Constraints:**
- Must maintain all existing functionality
- Cannot break existing HTML/CSS/JavaScript
- Must work from project root directory
- Keep static files in `src/web/` (no restructuring)

## Goals / Non-Goals

**Goals:**
1. Serve both static files and API endpoints from single Actix-web server
2. Eliminate Python dependency for local development
3. Single `cargo run` command to start application
4. Enable relative URLs in HTML forms (port-independent)
5. Production-ready static file serving with caching

**Non-Goals:**
- Embedding static files in binary (future enhancement)
- Asset preprocessing/bundling (keep simple HTML/CSS)
- CDN integration or edge caching
- Advanced compression or minification
- Hot reload for HTML changes (acceptable to refresh browser)

## Decisions

### Decision 1: Use `actix-files` crate for static file serving
**Choice:** Add `actix-files = "0.6"` dependency to serve files from filesystem

**Rationale:**
- Official Actix ecosystem crate, well-maintained
- Supports caching headers (Last-Modified, ETag)
- Handles MIME types automatically
- Production-ready with range requests, conditional GET
- Minimal configuration required

**Alternatives Considered:**
- `actix-web-static-files`: Embeds files in binary (overkill for development)
- Custom file handler: Reinventing the wheel, more code to maintain
- Keep Python server: Doesn't solve the complexity problem

### Decision 2: Route API endpoints before static files
**Choice:** Configure routes in order: `/health` → `/api/*` → `/*` (static)

**Rationale:**
- More specific routes matched first prevents conflicts
- API endpoints always take precedence over files
- Catch-all static route handles everything else
- Standard web server pattern

**Alternatives Considered:**
- Prefix static files with `/static/`: Requires HTML changes, less clean URLs
- Use different ports: Keeps complexity, defeats the purpose

### Decision 3: Serve static files from `./src/web` directory
**Choice:** Configure `actix-files` to serve from "./src/web" relative path

**Rationale:**
- No file restructuring needed
- Developers already familiar with this location
- Works when running from project root (`cargo run`)
- Clear separation of concerns (web assets vs Rust code)

**Alternatives Considered:**
- Move to `static/`: Requires file moves, breaks existing setup
- Absolute paths: Less portable across environments

### Decision 4: Update HTML forms to use relative URLs
**Choice:** Change `action="http://localhost:8080/api/login"` to `action="/api/login"`

**Rationale:**
- Works on any port or domain without changes
- Same-origin requests (no CORS needed)
- Standard web best practice
- More portable for deployment

**Alternatives Considered:**
- Keep absolute URLs: Brittle, requires changes for different environments
- JavaScript URL construction: Unnecessary complexity

### Decision 5: Enable caching with `use_last_modified(true)`
**Choice:** Configure actix-files with Last-Modified headers

**Rationale:**
- Browser caching improves performance
- 304 Not Modified responses reduce bandwidth
- Standard HTTP caching behavior
- No additional code needed

**Alternatives Considered:**
- No caching: Slower page loads, more server work
- Manual cache headers: More complex, reinventing built-in feature

### Decision 6: Set `index.html` as default index file
**Choice:** Configure `.index_file("index.html")` for directory requests

**Rationale:**
- `/` serves `index.html` automatically (login page)
- Standard web server behavior users expect
- No redirect needed

**Alternatives Considered:**
- Explicit `/index.html` in URLs: Less clean, not standard
- Custom redirect: Extra code for standard behavior

## Implementation Strategy

**Phase 1: Backend Setup**
1. Add `actix-files` dependency to `Cargo.toml`
2. Import `actix_files` in `main.rs`
3. Add static file service after API routes

**Phase 2: Configuration**
```rust
.service(
    fs::Files::new("/", "./src/web")
        .index_file("index.html")
        .use_last_modified(true)
)
```

**Phase 3: Frontend Updates**
- Update `index.html` form action to `/api/login`
- Update `create-user.html` form action to `/api/create-user`
- Update `user-info.html` fetch call to `/api/users/${userId}`

**Phase 4: Documentation**
- Update `src/web/README.md` to remove Python server instructions
- Emphasize single `cargo run` command
- Document working directory requirement

## Risks / Trade-offs

**Risk: Working directory dependency**
- **Description:** Application must run from project root for `./src/web` to resolve
- **Mitigation:** Document clearly, consider environment variable for path in future
- **Impact:** Low (standard practice to run from project root)

**Risk: No hot reload for HTML changes**
- **Description:** Browser refresh needed to see HTML/CSS changes (no rebuild needed)
- **Impact:** Low (acceptable for development, same as Python server)
- **Mitigation:** Browser auto-refresh tools if needed

**Trade-off: Static files served from filesystem vs embedded**
- **Choice:** Serve from disk
- **Rationale:** Development-friendly, no rebuild for HTML changes
- **Future:** Can embed for production deployment with `rust-embed`

**Trade-off: Security - serving all files in src/web/**
- **Risk:** Accidentally exposing non-public files
- **Mitigation:** Keep only public files in `src/web/`, don't store secrets there
- **Impact:** Low (current practice already safe)

## Testing Strategy

**Manual Testing:**
1. Start server: `cargo run`
2. Navigate to `http://localhost:8080/` → should see login page
3. Test form submission → should work with relative URLs
4. Verify API endpoints still work: `/api/login`, `/api/create-user`, `/api/users/{id}`
5. Test CSS loading: `http://localhost:8080/style.css`

**Integration Tests:**
- Verify health endpoint: `GET /health` → 200 OK
- Verify API still works: `POST /api/login` → correct response
- Verify static file: `GET /` → returns HTML content
- Verify 404 for missing files: `GET /nonexistent.html` → 404

**Browser Testing:**
- Login flow end-to-end
- Create user flow end-to-end
- Verify CSS applies correctly
- Test in Chrome, Firefox, Safari

## Future Enhancements

1. **Embedded Static Files:** Use `rust-embed` to compile files into binary for deployment
2. **Compression:** Enable gzip/brotli compression for text files
3. **Cache-Control Headers:** Add explicit cache TTL headers
4. **Content Security Policy:** Add security headers
5. **SPA Fallback:** Route all non-file requests to index.html if needed for future SPA

## Success Metrics

- ✅ Single `cargo run` starts both API and static serving
- ✅ No Python dependency needed
- ✅ All existing functionality works unchanged
- ✅ HTML forms use relative URLs
- ✅ Existing tests pass
- ✅ Documentation updated
