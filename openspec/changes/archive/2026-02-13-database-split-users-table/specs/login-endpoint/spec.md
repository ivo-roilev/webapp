## MODIFIED Requirements

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
