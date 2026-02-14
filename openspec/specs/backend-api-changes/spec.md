# Spec: Backend API Changes

## Purpose

This spec defines requirements for backend API endpoints to support minimal JavaScript form submission with plain text responses.

## Requirements

### Requirement: Create User Endpoint SHALL Use Explicit Path

The create user endpoint SHALL be renamed from `/api/users` to `/api/create-user` to avoid confusion with the GET user info endpoint.

#### Scenario: POST to /api/create-user succeeds
- **WHEN** client sends POST request to `/api/create-user` with valid user data
- **THEN** server creates user and returns appropriate response

#### Scenario: Old endpoint no longer accepts POST
- **WHEN** client sends POST request to `/api/users`
- **THEN** server returns 404 Not Found or 405 Method Not Allowed

### Requirement: Login Endpoint SHALL Return Plain Text User ID

The login endpoint SHALL return HTTP 200 OK with plain text user_id instead of JSON response upon successful authentication.

#### Scenario: Successful login returns user ID
- **WHEN** client sends POST to `/api/login` with valid credentials
- **THEN** server returns status 200 OK
- **AND** Content-Type header is `text/plain`
- **AND** response body contains the user_id as plain text (e.g., "1")

#### Scenario: Failed login returns error
- **WHEN** client sends POST to `/api/login` with invalid credentials
- **THEN** server returns status 401 Unauthorized
- **AND** response contains JSON error message

#### Scenario: Missing credentials return validation error
- **WHEN** client sends POST to `/api/login` with empty username or password
- **THEN** server returns status 400 Bad Request
- **AND** response contains JSON error message

### Requirement: Create User Endpoint SHALL Return Plain Text User ID

The create user endpoint SHALL return HTTP 200 OK with plain text user_id instead of JSON response upon successful user creation.

#### Scenario: Successful user creation returns user ID
- **WHEN** client sends POST to `/api/create-user` with valid user data
- **THEN** server returns status 200 OK
- **AND** Content-Type header is `text/plain`
- **AND** response body contains the user_id as plain text (e.g., "1")

#### Scenario: Duplicate username returns conflict
- **WHEN** client sends POST to `/api/create-user` with existing username
- **THEN** server returns status 409 Conflict
- **AND** response contains JSON error message

#### Scenario: Invalid user data returns validation error
- **WHEN** client sends POST to `/api/create-user` with invalid data
- **THEN** server returns status 400 Bad Request
- **AND** response contains JSON error message

### Requirement: Backend SHALL Accept Form-Encoded Data

Both login and create user endpoints SHALL accept `application/x-www-form-urlencoded` content type for form submissions.

#### Scenario: Form-encoded data is accepted
- **WHEN** client sends POST with Content-Type `application/x-www-form-urlencoded`
- **THEN** server successfully parses the form data
- **AND** processes the request normally

#### Scenario: User ID is positive integer
- **WHEN** server returns user_id after successful operation
- **THEN** user_id MUST be a positive integer (greater than 0)
