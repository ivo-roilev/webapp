## Context

The application has a MySQL user management database with a `users` table containing user credentials and profile information (designed in the 2026-02-10-database change). Currently, there is no service layer to expose the database functionality to client applications. The Rust RESTful service will bridge this gap by providing HTTP-based access to the database for user registration, authentication, and profile retrieval. The service must reliably handle concurrent requests, securely connect to MySQL, and provide clear JSON APIs to clients.

## Goals / Non-Goals

**Goals:**
- Design a RESTful API service in Rust that exposes three endpoints for user management
- Establish a normal, non-secure MySQL database connectivity with proper connection handling
- Define clear JSON request/response schemas for all three endpoints
- Ensure efficient handling of concurrent HTTP requests
- Simplify client integration by providing well-documented HTTP endpoints

**Non-Goals:**
- Store passwords securely (application will store plain-text as per database design)
- Implement authentication/authorization beyond username+password validation
- Build a web UI or admin dashboard
- Provide database schema migration tools (schema is already defined)
- Implement rate limiting or DDoS protection
- Support multi-database or database failover scenarios

## Decisions

**Decision 1: Use Actix-web framework for HTTP server**
- **Rationale**: Actix-web is a high-performance, production-ready Rust web framework with excellent concurrency support, built-in JSON serialization, and a large ecosystem. It provides async/await support for non-blocking I/O, critical for handling multiple concurrent requests to the database.
- **Alternatives Considered**: Axum (excellent choice, similar performance, slightly more modern), Rocket (simpler API but lower performance), plain Tokio (requires more boilerplate), Warp (functional style, smaller community)

**Decision 2: Use MySqlPool from either mysql_async or sqlx crate for database connectivity**
- **Rationale**: Connection pooling is essential to avoid opening a new database connection per request. MySqlPool manages a pool of reusable connections, improving performance and resource efficiency. sqlx provides compile-time query verification.
- **Alternatives Considered**: mysql_async with r2d2 pooling (mature, well-tested), direct mysql connections (inefficient, not recommended), tokio-mysql (deprecated)
- **Selection Method**: Will evaluate both during implementation - sqlx for type safety if compile-time verification is desired, mysql_async with r2d2 for runtime flexibility

**Decision 3: Request/Response JSON schema design**
- **Create User Endpoint**: Accept JSON with username, password, and optional fields (first_name, last_name, email, title, hobby). Return user_id as integer on success. Return error JSON with message on failure (duplicate username, validation).
- **Login Endpoint**: Accept JSON with username and password. Return `{"user_id": <integer>}` on success. Return error JSON with message on failure (user not found, invalid password).
- **Get User Info Endpoint**: Accept user_id as URL path parameter. Return formatted user information as a JSON object containing all user fields from the database.
- **Error Responses**: All endpoints return standard error format: `{"error": "<message>"}` with appropriate HTTP status codes (400 for bad request, 409 for conflict, 401 for unauthorized, 404 for not found, 500 for server error)

**Decision 4: Service listens on localhost:8080 by default**
- **Rationale**: Standard HTTP port for development. Environment variables will allow customization for deployment.
- **Alternatives Considered**: Hardcoded port (inflexible), auto-select available port (unclear for clients)

**Decision 5: No input validation at service layer (rely on database constraints)**
- **Rationale**: The MySQL schema already enforces constraints (VARCHAR length limits, UNIQUE on username). Service will attempt database operation and return database constraint violation errors. This minimizes code duplication.
- **Alternatives Considered**: Duplicate validation logic at service layer (adds complexity), client-side validation only (insufficient security)

**Decision 6: Single-threaded async Tokio runtime with Actix runtime**
- **Rationale**: Tokio provides efficient async/await based concurrency. Actix-web runs on top of Tokio and automatically handles multi-core scalability.
- **Alternatives Considered**: Manual thread pooling (unnecessary, Tokio handles it), blocking database calls (poor concurrency)

## Risks / Trade-offs

**Risk: Plain-text passwords in database** → Mitigation: Documented in database schema design as intentional. Service will not modify this behavior; it is a database-level decision. Future application updates can hash passwords at the database layer.

**Risk: Concurrent duplicate username registration** → Mitigation: MySQL UNIQUE constraint on username column enforces uniqueness at database level. Service returns 409 Conflict on constraint violation attempts. Application-level race condition is mitigated by database constraint.

**Risk: Database connection pool exhaustion under high load** → Mitigation: Pool size will be configurable. Monitor connection pool metrics. Add timeouts to pool acquisition.

**Risk: SQL injection** → Mitigation: Use parameterized queries (prepared statements) with sqlx or mysql_async. Never construct SQL strings directly with user input.

**Risk: Password exposure in logs** → Mitigation: Service will not log password values or include them in error messages. Debug logs will be disabled in production.

**Risk: Service crashes leave client requests hanging** → Mitigation: Implement graceful shutdown. Tokio runtime provides signal handling for cleanup.

**Trade-off: Runtime JSON serialization vs compile-time verification** → Accepted trade-off for development speed. sqlx offers compile-time verification but requires more setup; mysql_async offers runtime flexibility.

**Trade-off: No caching layer** → Acceptable for initial version. Database queries are lightweight for small user tables. Caching can be added later if performance becomes an issue.

**Open Questions:**
- Should the service support HTTPS/TLS? (Recommend yes for production, use reverse proxy or native Rust TLS): Answer - not for the first version.
- What should be the maximum connection pool size? (Recommend 10-20 for initial deployment, scale based on load testing): Answer - OK
- Should the service log all requests? (Recommend yes for debugging, exclude passwords from logs): Answer - yes.
- Should the Get User Info endpoint support multiple user IDs in a single request? (Recommend no for initial version, implement admin endpoints later): : Answer - no
