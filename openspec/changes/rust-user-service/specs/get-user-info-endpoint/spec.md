## ADDED Requirements

### Requirement: User Information Retrieval via HTTP GET
The system SHALL provide an HTTP GET endpoint at `/api/users/{user_id}` that retrieves the complete user profile information from the MySQL database by user ID.

#### Scenario: Successful retrieval of existing user
- **WHEN** a client sends GET request to `/api/users/{user_id}` with a valid user ID that exists in the database
- **THEN** the system returns HTTP 200 with JSON response containing all user fields (id, username, first_name, last_name, email, title, hobby, created_at, updated_at)

#### Scenario: Retrieve user with all profile fields populated
- **WHEN** a client sends GET request to `/api/users/{user_id}` for a user with all optional fields filled
- **THEN** the system returns complete user object including all optional fields (first_name, last_name, email, title, hobby)

#### Scenario: Retrieve user with sparse optional fields
- **WHEN** a client sends GET request to `/api/users/{user_id}` for a user with some optional fields null
- **THEN** the system returns user object with null values for empty optional fields

### Requirement: User ID Validation
The system SHALL validate that the user ID parameter is a valid integer and the requested user exists in the database.

#### Scenario: Request with non-existent user ID
- **WHEN** a client sends GET request to `/api/users/{user_id}` with a user ID that does not exist in the database
- **THEN** the system returns HTTP 404 Not Found with error JSON `{"error": "user not found"}`

#### Scenario: Request with invalid user ID format
- **WHEN** a client sends GET request to `/api/users/{user_id}` with non-numeric user ID (e.g., "abc")
- **THEN** the system returns HTTP 400 Bad Request with error JSON `{"error": "user_id must be a valid integer"}`

#### Scenario: Request with negative user ID
- **WHEN** a client sends GET request to `/api/users/{user_id}` with negative user ID
- **THEN** the system returns HTTP 400 Bad Request with error JSON `{"error": "user_id must be a positive integer"}`

### Requirement: JSON Response Format
The system SHALL return user information as a formatted JSON object containing all fields from the users table.

#### Scenario: Response includes all user fields
- **WHEN** a client successfully retrieves a user record
- **THEN** the response JSON contains: id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at

#### Scenario: Response excludes database passwords from logs
- **WHEN** service logs are enabled
- **THEN** user passwords are never logged or included in debug output

### Requirement: Database Connection Handling
The system SHALL handle database connection errors gracefully and return appropriate HTTP error responses.

#### Scenario: Database connection failure during user retrieval
- **WHEN** the database is unavailable and a client sends GET request to `/api/users/{user_id}`
- **THEN** the system returns HTTP 503 Service Unavailable with error JSON `{"error": "database connection failed"}`

#### Scenario: Database query error during user lookup
- **WHEN** a database error occurs while fetching user information
- **THEN** the system returns HTTP 500 Internal Server Error with error JSON `{"error": "server error"}`

### Requirement: HTTP GET Method
The system SHALL use the HTTP GET method for user information retrieval and be idempotent (no side effects).

#### Scenario: Multiple identical GET requests return same result
- **WHEN** a client sends multiple GET requests to `/api/users/{user_id}` with the same user ID
- **THEN** each request returns identical user data without modifying the database

#### Scenario: GET request does not create or modify user records
- **WHEN** a client sends GET request to `/api/users/{user_id}`
- **THEN** the user record in the database remains unchanged
