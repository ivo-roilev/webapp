## MODIFIED Requirements

### Requirement: User Information Retrieval via HTTP GET
The system SHALL provide an HTTP GET endpoint at `/api/users/{user_id}` that retrieves user profile information from the MySQL database by user ID and returns it as a human-readable text summary.

#### Scenario: Successful retrieval of existing user with all fields
- **WHEN** a client sends GET request to `/api/users/{user_id}` with a valid user ID that exists in the database and has all optional fields populated
- **THEN** the system returns HTTP 200 with Content-Type `text/plain` containing formatted text like `"Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"`

#### Scenario: Retrieve user with minimal profile (username only)
- **WHEN** a client sends GET request to `/api/users/{user_id}` for a user with no optional fields populated
- **THEN** the system returns HTTP 200 with Content-Type `text/plain` containing greeting with username (e.g., `"Hello jdoe, welcome!"`)

#### Scenario: Retrieve user with partial optional fields
- **WHEN** a client sends GET request to `/api/users/{user_id}` for a user with some optional fields null (e.g., has name and title but no email or hobby)
- **THEN** the system returns HTTP 200 with text omitting empty sections cleanly (e.g., `"Hello Software Engineer John Doe, welcome!"`)

#### Scenario: Retrieve user with hobby and email but no title
- **WHEN** a client sends GET request to `/api/users/{user_id}` for a user with name, email, and hobby but no title
- **THEN** the system returns HTTP 200 with text formatted as `"Hello John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"`

### Requirement: User ID Validation
The system SHALL validate that the user ID parameter is a valid integer and the requested user exists in the database, returning plain text error messages.

#### Scenario: Request with non-existent user ID
- **WHEN** a client sends GET request to `/api/users/{user_id}` with a user ID that does not exist in the database
- **THEN** the system returns HTTP 404 Not Found with Content-Type `text/plain` containing error message `"User with ID {user_id} not found"`

#### Scenario: Request with invalid user ID format
- **WHEN** a client sends GET request to `/api/users/{user_id}` with non-numeric user ID (e.g., "abc")
- **THEN** the system returns HTTP 400 Bad Request with Content-Type `text/plain` containing error message `"user_id must be a valid integer"`

#### Scenario: Request with negative user ID
- **WHEN** a client sends GET request to `/api/users/{user_id}` with negative user ID
- **THEN** the system returns HTTP 400 Bad Request with Content-Type `text/plain` containing error message `"user_id must be a positive integer"`

### Requirement: Plain Text Response Format
The system SHALL return user information as a human-readable plain text summary following the greeting format: `"Hello [Title] [Name], welcome! If we hear interesting news about [Hobby], we will let you know at [Email]!"` with sections omitted when fields are empty.

#### Scenario: Text format includes all components
- **WHEN** a client successfully retrieves a user with first_name, last_name, email, title, and hobby populated
- **THEN** the response text follows the format `"Hello Job Title FirstName LastName, welcome! If we hear interesting news about hobby activity, we will let you know at email@example.com!"`

#### Scenario: Name construction uses available fields
- **WHEN** retrieving a user with both first_name and last_name
- **THEN** the name portion is formatted as `"FirstName LastName"`
- **WHEN** retrieving a user with only first_name
- **THEN** the name portion uses just `"FirstName"`
- **WHEN** retrieving a user with only last_name
- **THEN** the name portion uses just `"LastName"`
- **WHEN** retrieving a user with neither first_name nor last_name
- **THEN** the name portion uses the username

#### Scenario: Email section formatting
- **WHEN** user has email field and hobby populated
- **THEN** the text includes email in the notification clause: `"If we hear interesting news about hobby, we will let you know at email@example.com!"`
- **WHEN** user has no email field
- **THEN** the text omits the email notification clause entirely

#### Scenario: Title and hobby section formatting
- **WHEN** user has title
- **THEN** the text includes title in the greeting: `"Hello Title Name, welcome!"`
- **WHEN** user has hobby
- **THEN** the text includes hobby notification: `"If we hear interesting news about hobby, we will let you know"`
- **WHEN** user has hobby and email
- **THEN** the text includes both in notification: `"If we hear interesting news about hobby, we will let you know at email!"`
- **WHEN** user has hobby but no email
- **THEN** the text includes hobby without email: `"If we hear interesting news about hobby, we will let you know!"`
- **WHEN** user has no title
- **THEN** the greeting uses name only: `"Hello Name, welcome!"`
- **WHEN** user has neither hobby nor email notification section
- **THEN** the text ends with the welcome greeting

#### Scenario: Response excludes sensitive fields from output
- **WHEN** a client successfully retrieves a user record
- **THEN** the response text does NOT include password, id, created_at, or updated_at fields
- AND passwords are never logged or included in debug output

### Requirement: Database Connection Handling
The system SHALL handle database connection errors gracefully and return appropriate plain text HTTP error responses.

#### Scenario: Database connection failure during user retrieval
- **WHEN** the database is unavailable and a client sends GET request to `/api/users/{user_id}`
- **THEN** the system returns HTTP 503 Service Unavailable with Content-Type `text/plain` containing error message `"Database connection failed"`

#### Scenario: Database query error during user lookup
- **WHEN** a database error occurs while fetching user information
- **THEN** the system returns HTTP 500 Internal Server Error with Content-Type `text/plain` containing error message `"Failed to fetch user"`
