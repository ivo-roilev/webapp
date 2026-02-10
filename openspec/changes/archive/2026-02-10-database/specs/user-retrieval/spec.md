## ADDED Requirements

### Requirement: Retrieve user by username
The system SHALL retrieve user information from the database based on a provided username and return complete user details.

#### Scenario: Successful user retrieval
- **WHEN** a valid username is provided that exists in the database
- **THEN** the system returns the user record including id, username, password, first_name, last_name, email, title, hobby, created_at, and updated_at

#### Scenario: User not found
- **WHEN** a username is provided that does not exist in the database
- **THEN** the system returns a "user not found" error or null result

#### Scenario: Empty or invalid username query
- **WHEN** an empty or invalid username string is provided for retrieval
- **THEN** the system rejects the request with a validation error
