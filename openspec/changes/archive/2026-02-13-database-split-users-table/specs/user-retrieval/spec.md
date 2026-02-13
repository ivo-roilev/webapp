## MODIFIED Requirements

### Requirement: Retrieve user by username
The system SHALL retrieve user information from the database based on a provided username by joining the `users`, `user_profiles`, and `user_metadata` tables.

#### Scenario: Successful user retrieval
- **WHEN** a valid username is provided that exists in the database
- **THEN** the system returns the aggregated user record including id, username, first_name, last_name, email, and all associated metadata

#### Scenario: User not found
- **WHEN** a username is provided that does not exist in the `users` table
- **THEN** the system returns a "user not found" error or null result

#### Scenario: Empty or invalid username query
- **WHEN** an empty or invalid username string is provided for retrieval
- **THEN** the system rejects the request with a validation error
