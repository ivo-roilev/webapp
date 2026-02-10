## ADDED Requirements

### Requirement: Create new user
The system SHALL accept user creation requests containing username, password, and user information (first_name, last_name, email, title, hobby) and persist this data in the database with a unique username constraint.

#### Scenario: Successful user creation
- **WHEN** a valid user creation request is submitted with unique username and all required information
- **THEN** the user is stored in the database with id, username, password, first_name, last_name, email, title, hobby, created_at, and updated_at fields

#### Scenario: Duplicate username rejection
- **WHEN** a user creation request is submitted with a username that already exists
- **THEN** the system rejects the request and returns an error indicating the username is unavailable

#### Scenario: User information validation
- **WHEN** a user creation request contains missing or invalid required fields
- **THEN** the system rejects the request and returns validation error messages

#### Scenario: Username length constraint
- **WHEN** a user creation request contains a username longer than 16 characters
- **THEN** the system rejects the request with a username length constraint error
