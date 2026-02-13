# Rust User Service API

A high-performance RESTful HTTP service for user management built with Rust, Actix-web, and MySQL.

## Project Overview

This service provides three core endpoints for user management:
- **Create User** - Register new users with optional profile information
- **Login** - Authenticate users and retrieve their IDs
- **Get User Info** - Retrieve complete user profile information

The service uses async/await with Tokio runtime, connection pooling with SQLx, and parameterized queries for SQL injection prevention.

## Technology Stack

- **Framework**: Actix-web 4 (high-performance HTTP server)
- **Database**: MySQL (production) & SQLite (testing) via SQLx AnyPool
- **Async Runtime**: Tokio 1 with full features
- **Serialization**: Serde + serde_json for JSON handling
- **Database Pooling**: SQLx built-in connection pool (min: 1, max: 10 connections)
- **Error Handling**: Custom error types with thiserror
- **Logging**: Dual-output system (local stdout via env_logger + remote HTTP via reqwest)
- **HTTP Client**: reqwest 0.11 with rustls-tls backend (connection pooling enabled)
- **Dates**: Chrono for timestamp handling

## API Specification

### 1. Create User - POST /api/users

Creates a new user account with the provided information.

**Request:**
```json
{
  "username": "john_doe",
  "password": "secure_password_123",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "title": "Software Engineer",
  "hobby": "Reading"
}
```

**Request Fields:**
- `username` (required, string, max 16 chars): Unique identifier for login
- `password` (required, string, max 255 chars): Account password (plain-text, non-hashed)
- `first_name` (optional, string, max 255 chars): User's first name
- `last_name` (optional, string, max 255 chars): User's last name
- `email` (optional, string, max 255 chars): User's email address
- `title` (optional, string, max 255 chars): Job title or role
- `hobby` (optional, string, max 255 chars): User's hobby or interest

**Success Response (HTTP 201 Created):**
```json
{
  "user_id": 42
}
```

**Error Responses:**

| Status | Error Code | Message | When |
|--------|-----------|---------|------|
| 400 | VALIDATION_ERROR | Username is required and must be max 16 characters | Missing/invalid username |
| 400 | VALIDATION_ERROR | Password is required and must be max 255 characters | Missing/invalid password |
| 400 | VALIDATION_ERROR | Field name must be max 255 characters | Optional field too long |
| 409 | DUPLICATE_USERNAME | Username 'username' already exists | Duplicate username |
| 503 | DATABASE_UNAVAILABLE | Database connection failed | Database down |
| 500 | INTERNAL_ERROR | Failed to create user | Other server errors |

**Example:**
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "password123",
    "email": "alice@example.com"
  }'
```

---

### 2. Login - POST /api/login

Authenticates a user with username and password credentials.

**Request:**
```json
{
  "username": "john_doe",
  "password": "secure_password_123"
}
```

**Request Fields:**
- `username` (required, string): The user's username
- `password` (required, string): The user's password (plain-text comparison)

**Success Response (HTTP 200 OK):**
```json
{
  "user_id": 42
}
```

**Error Responses:**

| Status | Error Code | Message | When |
|--------|-----------|---------|------|
| 400 | VALIDATION_ERROR | Username is required | Missing username |
| 400 | VALIDATION_ERROR | Password is required | Missing password |
| 401 | INVALID_CREDENTIALS | Invalid username or password | Wrong credentials |
| 503 | DATABASE_UNAVAILABLE | Database connection failed | Database down |
| 500 | INTERNAL_ERROR | Login failed | Other server errors |

**Example:**
```bash
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "alice",
    "password": "password123"
  }'
```

**Note:** Both username and password must match exactly. Passwords are compared as plain-text.

---

### 3. Get User Info - GET /api/users/{user_id}

Retrieves complete profile information for a user by ID.

**Path Parameters:**
- `user_id` (required, integer): The numeric user ID (must be positive)

**Success Response (HTTP 200 OK):**
```json
{
  "id": 42,
  "username": "john_doe",
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "title": "Software Engineer",
  "hobby": "Reading",
  "created_at": "2026-02-10T10:30:45.123456",
  "updated_at": "2026-02-10T14:45:20.654321"
}
```

**Response Fields:**
- `id`: User's numeric ID
- `username`: Unique username
- `first_name`: First name (may be null)
- `last_name`: Last name (may be null)
- `email`: Email address (may be null)
- `title`: Job title (may be null)
- `hobby`: Hobby or interest (may be null)
- `created_at`: Timestamp of account creation
- `updated_at`: Timestamp of last update

**Error Responses:**

| Status | Error Code | Message | When |
|--------|-----------|---------|------|
| 400 | VALIDATION_ERROR | user_id must be a positive integer | Invalid format |
| 404 | USER_NOT_FOUND | User with ID {id} not found | User doesn't exist |
| 503 | DATABASE_UNAVAILABLE | Database connection failed | Database down |
| 500 | INTERNAL_ERROR | Failed to fetch user | Other server errors |

**Example:**
```bash
curl http://localhost:8080/api/users/42
```

---

### 4. Health Check - GET /health

Simple endpoint to verify the server is running.

**Success Response (HTTP 200 OK):**
```json
{
  "status": "ok"
}
```

**Example:**
```bash
curl http://localhost:8080/health
```

---

## Environment Variables

Configure the service using environment variables:

```bash
# Database Configuration
DATABASE_HOST=localhost           # MySQL host (default: localhost)
DATABASE_PORT=3306               # MySQL port (default: 3306)
DATABASE_USER=root               # MySQL username (default: root)
DATABASE_PASSWORD=password        # MySQL password (default: password)
DATABASE_NAME=user_management   # Database name (default: user_management)

# Alternative: use complete URL
DATABASE_URL=mysql://user:pass@host:port/dbname

# Server Configuration
SERVER_HOST=127.0.0.1            # Bind address (default: 127.0.0.1)
SERVER_PORT=8080                 # Port (default: 8080)

# Logging
RUST_LOG=info                    # Log level (debug, info, warn, error)
LOGGER_URL=http://localhost:9090  # Remote logger service URL (optional)
```

## Database Schema

The service requires the following MySQL table:

```sql
CREATE TABLE users (
  id INT AUTO_INCREMENT PRIMARY KEY,
  username VARCHAR(16) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  first_name VARCHAR(255),
  last_name VARCHAR(255),
  email VARCHAR(255),
  title VARCHAR(255),
  hobby VARCHAR(255),
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  CHARSET utf8mb4
) ENGINE=InnoDB;
```

**Key Constraints:**
- `username`: UNIQUE constraint ensures no duplicate usernames
- `created_at`: Auto-set to current timestamp on insert
- `updated_at`: Auto-updated on any change
- UTF-8mb4 charset for international character support

---

## Build and Run Instructions

### Prerequisites

- Rust 1.93.0 or later
- MySQL 5.7+ server running
- Cargo package manager

### Build

```bash
cd src
cargo build --release
```

Output binary: `target/release/rust_user_service`

### Run

1. Create and configure `.env` file:

```bash
cp .env.example .env
# Edit .env with your database credentials
```

2. Ensure database and tables exist:

```bash
mysql -u root -p user_management < database/01_users_schema.sql
```

3. Start the service:

```bash
cargo run --release
# Or directly:
./target/release/rust_user_service
```

Server will listen on `127.0.0.1:8080` by default.

### Run Tests

```bash
cargo test --bin rust_user_service
```

Expected output: `test result: ok. 14 passed`

---

## Error Response Format

All error responses follow this standard format:

```json
{
  "error": "ERROR_CODE",
  "message": "Human-readable error description"
}
```

Status codes follow HTTP standards:
- **200**: Success (for login and get user info)
- **201**: Created (for successful user creation)
- **400**: Bad Request (validation errors)
- **401**: Unauthorized (authentication failures)
- **404**: Not Found (resource doesn't exist)
- **409**: Conflict (duplicate username)
- **503**: Service Unavailable (database errors)
- **500**: Internal Server Error (unexpected failures)

---

## Example .env Configuration

```bash
# Database
DATABASE_HOST=localhost
DATABASE_PORT=3306
DATABASE_USER=root
DATABASE_PASSWORD=my_secure_password
DATABASE_NAME=user_management

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Logging
RUST_LOG=info
```

---

## Testing Examples

### Test 1: Create User

```bash
# Request
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123",
    "first_name": "Test",
    "email": "test@example.com"
  }'

# Response
{"user_id":1}
```

### Test 2: Duplicate Username Error

```bash
# Request (reuse same username)
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "differentpass"
  }'

# Response
HTTP 409 Conflict
{"error":"DUPLICATE_USERNAME","message":"Username 'testuser' already exists"}
```

### Test 3: Login Success

```bash
# Request
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123"
  }'

# Response
{"user_id":1}
```

### Test 4: Login Failure

```bash
# Request (wrong password)
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "wrongpassword"
  }'

# Response
HTTP 401 Unauthorized
{"error":"INVALID_CREDENTIALS","message":"Invalid username or password"}
```

### Test 5: Get User Info

```bash
# Request
curl http://localhost:8080/api/users/1

# Response
{
  "id": 1,
  "username": "testuser",
  "first_name": "Test",
  "last_name": null,
  "email": "test@example.com",
  "title": null,
  "hobby": null,
  "created_at": "2026-02-10T10:30:45.123456",
  "updated_at": "2026-02-10T10:30:45.654321"
}
```

### Test 6: UTF-8 Support

```bash
# Request with Unicode characters
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "日本語ユーザー",
    "password": "test123",
    "first_name": "日本",
    "hobby": "🎭🎪"
  }'

# Response
{"user_id":2}
```

---

## Implementation Details

### Connection Pooling

- **Min connections**: 1 (maintained at all times)
- **Max connections**: 10 (maximum concurrent connections)
- **Acquire timeout**: 30 seconds (max wait time for connection)
- **Charset**: UTF-8mb4 (supports full Unicode range)

### Validation

- **Username**: Required, 1-16 characters
- **Password**: Required, 1-255 characters
- **Optional fields**: Max 255 characters each
- All queries use parameterized statements to prevent SQL injection

### Dual Logging System

- **Local stdout**: Via env_logger (configured with `RUST_LOG`)
- **Remote HTTP**: Non-blocking async delivery to logger service (configured with `LOGGER_URL`)
- Request logging via actix middleware
- Error logging without exposing sensitive data
- User context included in all log events
- System operations logged with "SYSTEM" identifier
- Graceful degradation if logger service unavailable

### Error Handling

- Database errors mapped to appropriate HTTP status codes
- Duplicate username constraint violation caught and mapped to 409 Conflict
- Connection failures return 503 Service Unavailable
- All errors include descriptive messages

---

## Dual Logging Architecture

The service implements a dual-output logging system that simultaneously writes to:
1. **Local stdout** - For development debugging and local monitoring
2. **Remote logger service** - For centralized log aggregation (optional)

### Logger Service Integration

**Configuration:**
```bash
# Optional - service works without logger
LOGGER_URL=http://localhost:9090
```

**Features:**
- Non-blocking async delivery (fire-and-forget with tokio::spawn)
- Connection pooling via shared reqwest::Client
- Structured JSON payloads with timestamp, level, app, user, message
- Graceful degradation (service continues if logger unavailable)
- No impact on request latency or user response times

### Log Macro API

The service provides four logging macros for different severity levels:

```rust
// Info level - general informational events
log_info!(&http_client, "create_user", username, "Creating new user");

// Error level - error conditions
log_error!(&http_client, "login", username, "Database error: {}", err);

// Warning level - warning conditions
log_warn!(&http_client, "get_user_info", user_id, "User {} not found", user_id);

// Debug level - detailed diagnostic information
log_debug!(&http_client, "create_user", username, "Validating user data");
```

**Macro Parameters:**
1. `&http_client` - Reference to Arc<reqwest::Client> from AppState
2. `"app"` - Function/operation name for log filtering
3. `user` - User identifier (username, user_id, or "SYSTEM")
4. `"message"` - Format string for log message
5. `args...` - Optional format arguments

### Log Format

**Local stdout:**
```
[2026-02-12T10:30:00Z] [INFO] [create_user] [alice] Creating new user
[2026-02-12T10:30:05Z] [ERROR] [login] [bob] Invalid password
[2026-02-12T10:30:10Z] [INFO] [main] [SYSTEM] Starting HTTP server on 127.0.0.1:8080
```

**Remote HTTP payload:**
```json
{
  "timestamp": "2026-02-12T10:30:00Z",
  "level": "info",
  "app": "create_user",
  "user": "alice",
  "message": "Creating new user"
}
```

### User Context in Logs

- **User operations**: Username preferred over user_id when available
- **System operations**: "SYSTEM" identifier for startup, shutdown, server events
- **Anonymous operations**: Empty string converts to no user field

**Example log calls:**
```rust
// With username
log_info!(&state.http_client, "create_user", payload.username, "User created with ID: {}", user_id);

// With user_id when username not available
log_info!(&state.http_client, "get_user_info", user_id.to_string(), "Fetching user info");

// System operation
log_info!(&http_client, "main", "SYSTEM", "Starting HTTP server on {}:{}", host, port);
```

### Performance Characteristics

- **Non-blocking**: HTTP logging runs in spawned tasks, never blocks handlers
- **Connection reuse**: Single Arc<Client> shared across all requests
- **Fire-and-forget**: Logging failures don't propagate to user requests
- **Minimal overhead**: Async spawning adds ~microseconds, not milliseconds

---

## Security Notes

- **SQL Injection**: Prevented via parameterized queries (sqlx::query with bind parameters)
- **Passwords**: Stored as plain-text (design requirement, not production-recommended)
- **Authentication**: Plain-text comparison (no hashing, design requirement)
- **Connection**: Non-TLS MySQL connection (per design specification)
- **Validation**: Field length constraints enforced at application level

---

## Performance Characteristics

- Async/await for non-blocking I/O
- Connection pooling reduces database connection overhead
- Tokio runtime for efficient concurrency
- Parameterized queries benefit from MySQL query caching

---

## Build Release Binary

```bash
cargo build --release
```

The optimized binary will be at: `target/release/rust_user_service`

This version includes:
- All optimizations enabled
- No debug symbols
- Smaller binary size
- Better runtime performance

---

## Test Release Binary with Database

```bash
# Start the service
./target/release/rust_user_service &

# Run health check
curl http://localhost:8080/health

# Test all endpoints
# (See Testing Examples section above)

# Stop the service
kill %1
```

---

## Project Structure

```
src/
├── Cargo.toml          # Project manifest with dependencies
├── .env               # Environment variable template
├── database/          # Database schema files
│   └── 01_users_schema.sql
└── rust/
    ├── README.md      # This file
    ├── main.rs        # HTTP server and handlers
    ├── db.rs          # Database connection and queries
    ├── logger.rs      # Dual-logging module with macro API
    └── user_info_formatter.rs  # User info text formatting
```

---

## Dependencies

- `actix-web` 4: HTTP framework
- `serde`/`serde_json` 1: JSON serialization
- `tokio` 1: Async runtime
- `sqlx` 0.7: Type-safe async database driver
- `reqwest` 0.11: HTTP client with rustls-tls backend (for logger integration)
- `chrono` 0.4: DateTime handling
- `log`/`env_logger`: Logging infrastructure
- `thiserror` 1: Error handling
- `dotenv` 0.15: Environment configuration

---

**API Version**: 1.0
**Last Updated**: February 12, 2026
**Author**: GitHub Copilot
