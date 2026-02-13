## MODIFIED Requirements

### Requirement: Create new user
The system SHALL accept user creation requests containing username, password, and user information (first_name, last_name, email, title, hobby) and persist this data in the database across core, profile, and metadata tables using a transaction.

#### Scenario: Successful user creation
- **WHEN** a valid user creation request is submitted with unique username and all required information
- **THEN** the core auth data is stored in `users`, profile data in `user_profiles`, and additional info in `user_metadata` atomically

#### Scenario: Duplicate username rejection
- **WHEN** a user creation request is submitted with a username that already exists in the `users` table
- **THEN** the system rejects the request and returns an error indicating the username is unavailable

#### Scenario: User information validation
- **WHEN** a user creation request contains missing or invalid required fields
- **THEN** the system rejects the request and returns validation error messages, ensuring no tables are updated

#### Scenario: Username length constraint
- **WHEN** a user creation request contains a username longer than 16 characters
- **THEN** the system rejects the request with a username length constraint error
