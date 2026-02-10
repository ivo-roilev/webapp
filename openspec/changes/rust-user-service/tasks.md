## 1. Project Setup and Dependencies

- [x] 1.1 Initialize new Rust project with Cargo
- [x] 1.2 Add Actix-web framework dependency (latest stable)
- [x] 1.3 Add MySQL async driver dependency (sqlx or mysql_async)
- [x] 1.4 Add connection pool dependency (r2d2 for mysql_async or built-in for sqlx)
- [x] 1.5 Add serde for JSON serialization/deserialization
- [x] 1.6 Add tokio async runtime dependency
- [x] 1.7 Add dotenv for environment variable loading
- [x] 1.8 Configure Cargo.toml with appropriate feature flags
- [x] 1.9 Set up .env file with example database credentials

## 2. Database Connection Layer

- [x] 2.1 Create database module with connection pool initialization
- [x] 2.2 Implement MySqlPool setup using environment variables
- [x] 2.3 Configure connection pool size (min/max connections)
- [x] 2.4 Add connection timeout configuration
- [x] 2.5 Implement database connection error handling
- [x] 2.6 Create database query functions for user operations
- [x] 2.7 Implement parameterized queries to prevent SQL injection
- [x] 2.8 Add UTF-8 charset configuration for MySQL connections
- [ ] 2.9 Implement connection retry logic for initial startup

## 3. HTTP Server Setup

- [x] 3.1 Create main.rs with Actix-web application setup
- [x] 3.2 Configure HTTP server to listen on localhost:8080 (configurable via env var)
- [x] 3.3 Set up async Tokio runtime for the application
- [x] 3.4 Implement server startup with database connection pool
- [x] 3.5 Add graceful shutdown handling
- [x] 3.6 Implement request logging middleware
- [x] 3.7 Configure error handling middleware
- [x] 3.8 Set up CORS middleware if needed

## 4. Create User Endpoint Implementation

- [ ] 4.1 Create request struct for user creation (username, password, optional fields)
- [ ] 4.2 Create response struct for user creation response
- [x] 4.3 Implement POST /api/users route handler
- [x] 4.4 Add username validation (required, max 16 chars)
- [x] 4.5 Add password validation (required, max 255 chars)
- [x] 4.6 Add optional field validation (first_name, last_name, email, title, hobby - max 255 chars)
- [x] 4.7 Implement database INSERT query for new user
- [x] 4.8 Handle UNIQUE constraint violation for duplicate username
- [x] 4.9 Return user_id on successful creation (HTTP 201)
- [x] 4.10 Return appropriate error responses with proper HTTP status codes

## 5. Login Endpoint Implementation

- [x] 5.1 Create request struct for login (username, password)
- [x] 5.2 Create response struct for login response (user_id only)
- [x] 5.3 Implement POST /api/login route handler
- [x] 5.4 Add username validation (required field check)
- [x] 5.5 Add password validation (required field check)
- [x] 5.6 Implement database SELECT query to find user by username
- [x] 5.7 Implement password comparison logic (plain-text matching)
- [x] 5.8 Return user_id on successful authentication (HTTP 200)
- [x] 5.9 Return 401 Unauthorized for invalid credentials
- [x] 5.10 Ensure password is not included in response or logs

## 6. Get User Information Endpoint Implementation

- [x] 6.1 Create response struct for user info (all user fields)
- [x] 6.2 Implement GET /api/users/{user_id} route handler
- [x] 6.3 Add user_id parameter validation (must be positive integer)
- [x] 6.4 Implement database SELECT query to find user by ID
- [x] 6.5 Return complete user information as JSON on success (HTTP 200)
- [x] 6.6 Return 404 Not Found for non-existent user ID
- [x] 6.7 Return 400 Bad Request for invalid user_id format
- [x] 6.8 Ensure idempotency (GET requests do not modify data)

## 7. Error Handling and Validation

- [x] 7.1 Implement JSON validation error responses
- [x] 7.2 Add Content-Type validation for POST requests
- [x] 7.3 Create custom error types for different error scenarios
- [x] 7.4 Implement error response serialization to JSON
- [x] 7.5 Add database error mapping to HTTP status codes
- [x] 7.6 Implement request validation middleware
- [x] 7.7 Add proper HTTP status codes for all endpoints
- [x] 7.8 Implement logging of errors (without exposing sensitive data)

## 8. Input Validation and Security

- [x] 8.1 Ensure all SQL queries use parameterized statements
- [x] 8.2 Implement field length validation before database operations
- [x] 8.3 Add JSON structure validation
- [ ] 8.4 Implement request size limits
- [ ] 8.5 Add timeout handling for database queries
- [ ] 8.6 Configure connection pool leak detection
- [ ] 8.7 Test SQL injection prevention with malicious inputs

## 9. Testing

- [ ] 9.1 Set up test module structure
- [ ] 9.2 Write unit tests for user creation endpoint
- [ ] 9.3 Write unit tests for login endpoint
- [ ] 9.4 Write unit tests for get user info endpoint
- [ ] 9.5 Write integration tests with test database
- [ ] 9.6 Test duplicate username rejection
- [ ] 9.7 Test invalid credential handling
- [ ] 9.8 Test missing required field validation
- [ ] 9.9 Test field length constraints
- [ ] 9.10 Test JSON parsing error handling
- [ ] 9.11 Test database connection failure handling
- [ ] 9.12 Test UTF-8 character handling
- [ ] 9.13 Run all tests and verify they pass

## 10. Documentation and Deployment

- [ ] 10.1 Create README.md with API documentation
- [ ] 10.2 Document all three endpoint specifications (method, path, request/response)
- [ ] 10.3 Add example curl commands for each endpoint
- [ ] 10.4 Document environment variable configuration
- [ ] 10.5 Document database schema requirements
- [ ] 10.6 Add build and run instructions
- [ ] 10.7 Document error response formats
- [ ] 10.8 Create example .env configuration file
- [ ] 10.9 Build release binary (cargo build --release)
- [ ] 10.10 Test release binary with actual MySQL database
