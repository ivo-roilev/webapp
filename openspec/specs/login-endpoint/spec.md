# login-endpoint Specification

## Purpose
TBD - created by archiving change rust-user-service. Update Purpose after archive.
## Requirements
### Requirement: User Authentication via HTTP POST
The system SHALL provide an HTTP POST endpoint at `/api/login` that accepts username and password credentials in JSON format and validates them against the core `users` table.

#### Scenario: Successful login with valid credentials
- **WHEN** a client sends POST request to `/api/login` with correct username and password
- **THEN** the system returns HTTP 200 with JSON response `{"user_id": <integer>}` after querying the `users` table

#### Scenario: Request missing username field
- **WHEN** a client sends POST request to `/api/login` without username field
- **THEN** the system returns HTTP 400 with error JSON `{"error": "username is required"}`

#### Scenario: Request missing password field
- **WHEN** a client sends POST request to `/api/login` without password field
- **THEN** the system returns HTTP 400 with error JSON `{"error": "password is required"}`

### Requirement: Username Validation
The system SHALL validate that the provided username exists in the database and return an error if the user is not found.

#### Scenario: Login with non-existent username
- **WHEN** a client sends POST request to `/api/login` with a username that does not exist in the database
- **THEN** the system returns HTTP 401 Unauthorized with error JSON `{"error": "invalid username or password"}`

### Requirement: Password Validation
The system SHALL validate that the provided password matches the stored password for the given username and return an error if authentication fails.

#### Scenario: Login with incorrect password
- **WHEN** a client sends POST request to `/api/login` with correct username but incorrect password
- **THEN** the system returns HTTP 401 Unauthorized with error JSON `{"error": "invalid username or password"}`

### Requirement: JSON Request Format
The system SHALL accept login credentials as JSON with username and password fields.

#### Scenario: Valid JSON structure with both fields
- **WHEN** a client sends POST request with JSON: `{"username": "john_doe", "password": "secret123"}`
- **THEN** the system parses the JSON and validates the credentials

#### Scenario: Malformed JSON
- **WHEN** a client sends POST request with invalid JSON syntax
- **THEN** the system returns HTTP 400 with error JSON `{"error": "invalid JSON format"}`

#### Scenario: Request missing Content-Type header
- **WHEN** a client sends POST request without Content-Type: application/json header
- **THEN** the system returns HTTP 400 with error JSON `{"error": "Content-Type must be application/json"}`

### Requirement: Database Connection Handling
The system SHALL handle database connection errors gracefully and return appropriate HTTP error responses.

#### Scenario: Database connection failure during login
- **WHEN** the database is unavailable and a client sends POST request to `/api/login`
- **THEN** the system returns HTTP 503 Service Unavailable with error JSON `{"error": "database connection failed"}`

#### Scenario: Database query error during password lookup
- **WHEN** a database error occurs while fetching user credentials
- **THEN** the system returns HTTP 500 with error JSON `{"error": "server error"}`

### Requirement: Return User ID on Success
The system SHALL return only the user ID (as an integer) on successful authentication, without exposing password or other sensitive information.

#### Scenario: Successful login returns only user_id
- **WHEN** a client successfully authenticates with valid credentials
- **THEN** the response contains only `{"user_id": <integer>}` with no password or email information

