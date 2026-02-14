# Spec: Backend API Changes

## ADDED Requirements

### Requirement: Create User Endpoint SHALL Use Explicit Path

The create user endpoint SHALL be renamed from `/api/users` to `/api/create-user` to avoid confusion with the GET user info endpoint.

#### Scenario: POST to /api/create-user succeeds
- **WHEN** client sends POST request to `/api/create-user` with valid user data
- **THEN** server creates user and returns appropriate response

#### Scenario: Old endpoint no longer accepts POST
- **WHEN** client sends POST request to `/api/users`
- **THEN** server returns 404 Not Found or 405 Method Not Allowed

### Requirement: Login Endpoint SHALL Return HTTP 303 Redirect

The login endpoint SHALL return HTTP 303 See Other status with Location header instead of JSON response upon successful authentication.

#### Scenario: Successful login returns redirect
- **WHEN** client sends POST to `/api/login` with valid credentials
- **THEN** server returns status 303 See Other
- **AND** Location header contains `/user-info.html?user_id={id}`

#### Scenario: Failed login returns error without redirect
- **WHEN** client sends POST to `/api/login` with invalid credentials
- **THEN** server returns status 401 Unauthorized
- **AND** no Location header is present

#### Scenario: Missing credentials return validation error
- **WHEN** client sends POST to `/api/login` with empty username or password
- **THEN** server returns status 400 Bad Request
- **AND** no Location header is present

### Requirement: Create User Endpoint SHALL Return HTTP 303 Redirect

The create user endpoint SHALL return HTTP 303 See Other status with Location header instead of JSON response upon successful user creation.

#### Scenario: Successful user creation returns redirect
- **WHEN** client sends POST to `/api/create-user` with valid user data
- **THEN** server returns status 303 See Other
- **AND** Location header contains `/user-info.html?user_id={id}`

#### Scenario: Duplicate username returns conflict without redirect
- **WHEN** client sends POST to `/api/create-user` with existing username
- **THEN** server returns status 409 Conflict
- **AND** no Location header is present

#### Scenario: Invalid user data returns validation error
- **WHEN** client sends POST to `/api/create-user` with invalid data
- **THEN** server returns status 400 Bad Request
- **AND** no Location header is present

### Requirement: Redirect Location SHALL Include User ID as Query Parameter

When redirecting after successful login or user creation, the Location header SHALL include the user_id as a query parameter.

#### Scenario: Redirect URL includes user_id
- **WHEN** server returns 303 redirect after successful operation
- **THEN** Location header matches pattern `/user-info.html?user_id={positive_integer}`

#### Scenario: User ID is positive integer
- **WHEN** server generates redirect URL
- **THEN** user_id query parameter MUST be a positive integer (greater than 0)

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no existing requirements are being removed.
