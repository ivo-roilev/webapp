# create-user-endpoint Specification

## Purpose
TBD - created by archiving change rust-user-service. Update Purpose after archive.
## Requirements
### Requirement: User Registration via HTTP POST
The system SHALL provide an HTTP POST endpoint at `/api/users` that accepts user registration data in JSON format and creates a new user record in the MySQL database.

#### Scenario: Successful user creation with required fields
- **WHEN** a client sends POST request to `/api/users` with JSON body containing username and password
- **THEN** the system creates a new user record and returns HTTP 201 with JSON response `{"user_id": <integer>}`

#### Scenario: User creation with all optional fields
- **WHEN** a client sends POST request to `/api/users` with username, password, first_name, last_name, email, title, and hobby
- **THEN** the system creates a new user record with all provided fields and returns HTTP 201 with the new user_id

#### Scenario: Request missing required username field
- **WHEN** a client sends POST request without username field
- **THEN** the system returns HTTP 400 with error JSON `{"error": "username is required"}`

#### Scenario: Request missing required password field
- **WHEN** a client sends POST request without password field
- **THEN** the system returns HTTP 400 with error JSON `{"error": "password is required"}`

### Requirement: Username Uniqueness Validation
The system SHALL enforce unique usernames and reject any attempt to create a user with a duplicate username.

#### Scenario: Duplicate username attempt
- **WHEN** a client sends POST request to `/api/users` with a username that already exists in the database
- **THEN** the system returns HTTP 409 Conflict with error JSON `{"error": "username already exists"}`

### Requirement: Field Length Constraints
The system SHALL validate that all user fields conform to the database schema length limits and reject requests that exceed these limits.

#### Scenario: Username exceeds 16 character limit
- **WHEN** a client sends POST request with username longer than 16 characters
- **THEN** the system returns HTTP 400 with error JSON `{"error": "username must not exceed 16 characters"}`

#### Scenario: Password exceeds 255 character limit
- **WHEN** a client sends POST request with password longer than 255 characters
- **THEN** the system returns HTTP 400 with error JSON `{"error": "password must not exceed 255 characters"}`

#### Scenario: Optional field (first_name) exceeds 255 character limit
- **WHEN** a client sends POST request with first_name longer than 255 characters
- **THEN** the system returns HTTP 400 with error JSON `{"error": "first_name must not exceed 255 characters"}`

### Requirement: JSON Request Format
The system SHALL accept user registration data as JSON with the following structure:

#### Scenario: Valid JSON structure with required fields
- **WHEN** a client sends POST request with JSON: `{"username": "john_doe", "password": "secret123"}`
- **THEN** the system parses the JSON and processes the registration

#### Scenario: Malformed JSON
- **WHEN** a client sends POST request with invalid JSON syntax
- **THEN** the system returns HTTP 400 with error JSON `{"error": "invalid JSON format"}`

#### Scenario: Request missing Content-Type header
- **WHEN** a client sends POST request without Content-Type: application/json header
- **THEN** the system returns HTTP 400 with error JSON `{"error": "Content-Type must be application/json"}`

### Requirement: Database Connection Handling
The system SHALL handle database connection errors gracefully and return appropriate HTTP error responses.

#### Scenario: Database connection failure
- **WHEN** the database is unavailable and a user sends POST request to `/api/users`
- **THEN** the system returns HTTP 503 Service Unavailable with error JSON `{"error": "database connection failed"}`

#### Scenario: Database query error
- **WHEN** a database constraint violation occurs during insert (other than duplicate username)
- **THEN** the system returns HTTP 500 with error JSON `{"error": "server error"}`

