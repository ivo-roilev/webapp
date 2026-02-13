# mysql-database-integration Specification

## Purpose
TBD - created by archiving change rust-user-service. Update Purpose after archive.

## Requirements

### Requirement: Connection to MySQL or SQLite Database

The system SHALL establish and maintain a connection to a MySQL database (production) or SQLite database (testing) using a database pool abstraction that supports both connection types.

**Implementation Note**: This requirement extends the original MySQL-only connection to support both MySQL (production) and SQLite (testing) via sqlx::AnyPool, enabling fast in-memory testing without external MySQL dependency.

#### Scenario: Service starts and connects to MySQL database in production
- **WHEN** the Rust service starts up with MySQL DATABASE_URL configured
- **THEN** the system establishes a connection pool to the configured MySQL database using AnyPool
- **AND** the pool type is transparently MySQL without code changes

#### Scenario: Test environment connects to in-memory SQLite
- **WHEN** test code calls Database::new_test()
- **THEN** the system establishes a connection pool to an in-memory SQLite database using AnyPool
- **AND** the schema is created inline with SQLite-compatible SQL
- **AND** the connection is isolated per test instance

#### Scenario: Connection uses environment variables for configuration
- **WHEN** the service reads environment variables at startup
- **THEN** the system retrieves database host, port, username, password, and database name from environment variables
- **AND** constructs appropriate connection string for MySQL or SQLite based on URL scheme

#### Scenario: Connection pool is reused across requests
- **WHEN** multiple HTTP requests are processed
- **THEN** the system reuses database connections from the pool rather than creating new connections per request
- **AND** this behavior is consistent for both MySQL and SQLite pools

### Requirement: Database Pool SHALL support both MySQL and SQLite

The Database struct SHALL use sqlx::AnyPool to abstract over MySQL (production) and SQLite (testing) connection pools, allowing the same code to work with both database types.

#### Scenario: Production constructor creates MySQL pool
- **WHEN** Database::new() is called with MySQL DATABASE_URL
- **THEN** AnyPool connects to MySQL database
- **AND** production behavior is unchanged from MySqlPool implementation
- **AND** queries work identically to previous MySqlPool code

#### Scenario: Test constructor creates SQLite pool
- **WHEN** Database::new_test() is called in test code
- **THEN** AnyPool connects to "sqlite::memory:" in-memory database
- **AND** test schema is created with SQLite-compatible SQL
- **AND** CRUD operations work identically to MySQL within test constraints

#### Scenario: Query syntax is database-agnostic
- **WHEN** application code executes parameterized queries
- **THEN** queries use standard SQL that works on both MySQL and SQLite
- **AND** parameter binding uses ? placeholders
- **AND** no conditional compilation needed for query code

#### Scenario: Schema differences are isolated to setup
- **WHEN** test database is initialized
- **THEN** SQLite schema uses AUTOINCREMENT instead of AUTO_INCREMENT
- **AND** SQLite schema uses INTEGER instead of INT
- **AND** application query code remains unchanged

### Requirement: Test constructor SHALL provide isolated database instances

The Database struct SHALL include a new_test() constructor (test-only) that creates an isolated in-memory SQLite database with schema initialization.

#### Scenario: Test constructor is only available in test builds
- **WHEN** production code compiles
- **THEN** Database::new_test() method is not available
- **AND** method is gated behind #[cfg(test)] attribute

#### Scenario: Each test gets isolated database
- **WHEN** multiple tests call Database::new_test()
- **THEN** each call creates separate in-memory SQLite instance
- **AND** data in one test does not affect other tests
- **AND** tests can run in parallel without conflicts

#### Scenario: Test schema is created automatically
- **WHEN** Database::new_test() is called
- **THEN** users table is created with SQLite-compatible schema
- **AND** schema includes all columns from production schema (id, username, password, optional fields)
- **AND** UNIQUE constraint on username is enforced

#### Scenario: Test database supports all CRUD operations
- **WHEN** test code calls create_user(), get_user_by_username(), get_user_by_id()
- **THEN** all operations work identically to MySQL
- **AND** constraint violations (duplicate username) are detected
- **AND** results are returned in same format as MySQL

### Requirement: Database error handling SHALL work with both connection types

The Database error handling SHALL correctly map both MySQL and SQLite errors to DatabaseError enum variants.

#### Scenario: Duplicate key error detected in SQLite
- **WHEN** test attempts to insert duplicate username in SQLite
- **THEN** Database maps SQLite UNIQUE constraint error to DatabaseError::DuplicateUsername
- **AND** handler returns HTTP 409 Conflict (same as MySQL)

#### Scenario: Not found error detected in SQLite
- **WHEN** test queries non-existent username in SQLite
- **THEN** Database maps sqlx::Error::RowNotFound to DatabaseError::UserNotFound
- **AND** handler returns HTTP 404 Not Found (same as MySQL)

#### Scenario: Connection error handling is consistent
- **WHEN** database connection fails (MySQL unreachable or SQLite memory allocation fails)
- **THEN** Database maps connection error to DatabaseError::ConnectionError
- **AND** error message includes relevant details

### Requirement: Access to Users Table Schema
The system SHALL access the users table with the following columns as defined in the database schema: id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at.

#### Scenario: Query returns all user columns
- **WHEN** the system executes a SELECT query on the users table
- **THEN** the result includes all columns from the schema: id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at

### Requirement: User Creation in Database
The system SHALL insert new user records into the users table with the provided registration data.

#### Scenario: Insert new user with required fields
- **WHEN** the service receives a CREATE USER request with username and password
- **THEN** the system inserts a new row into the users table with the provided username and password, auto-generating id and timestamps

#### Scenario: Insert new user with optional fields
- **WHEN** the service receives a CREATE USER request with optional fields (first_name, last_name, email, title, hobby)
- **THEN** the system inserts all provided optional fields into the corresponding columns in the users table

#### Scenario: UNIQUE constraint on username is enforced
- **WHEN** the system attempts to insert a user with a username that already exists
- **THEN** the database returns a constraint violation error which the service handles and returns to the client

### Requirement: User Retrieval by Username
The system SHALL query the users table to retrieve user records by username for authentication purposes.

#### Scenario: Query user by username for authentication
- **WHEN** the LOGIN endpoint receives a request with username and password
- **THEN** the system executes a SELECT query to find the user record where username matches

#### Scenario: Password comparison from database
- **WHEN** a user record is retrieved by username
- **THEN** the system compares the provided plain-text password with the stored password field

### Requirement: User Retrieval by ID
The system SHALL query the users table to retrieve user records by their unique ID.

#### Scenario: Query user by ID
- **WHEN** the GET USER INFO endpoint receives a request with user ID
- **THEN** the system executes a SELECT query to find the user record where id matches

### Requirement: SQL Injection Prevention
The system SHALL use parameterized queries (prepared statements) to prevent SQL injection attacks.

#### Scenario: User input in queries uses parameterization
- **WHEN** the system constructs database queries with user-provided data (username, password, user_id)
- **THEN** the query uses parameterized statement placeholders rather than string concatenation

### Requirement: Database Connection Error Handling
The system SHALL handle database connection failures and timeouts gracefully.

#### Scenario: Connection timeout during query execution
- **WHEN** a database query does not respond within the configured timeout
- **THEN** the system releases the connection and returns an error response to the client

#### Scenario: Database unavailable at startup
- **WHEN** the service attempts to start but the database is unreachable
- **THEN** the system logs an error and either retries or exits with clear error message

### Requirement: Connection Pool Configuration
The system SHALL use a connection pool to efficiently manage database connections.

#### Scenario: Connection pool is initialized with configurable size
- **WHEN** the service starts
- **THEN** the system creates a connection pool with configurable minimum and maximum connection limits

#### Scenario: Connections are recycled when idle
- **WHEN** a database connection is idle for a configured duration
- **THEN** the system may close and remove the connection from the pool to free resources

### Requirement: Charset and Encoding
The system SHALL ensure that MySQL connections use UTF-8 encoding to properly store and retrieve user data with special characters.

#### Scenario: UTF-8 data is stored and retrieved correctly
- **WHEN** a user registers with a name containing non-ASCII characters
- **THEN** the system stores and retrieves the data correctly maintaining the original characters

### Requirement: Transaction Handling
The system SHALL handle each database operation (insert, select) as an atomic transaction.

#### Scenario: User creation is atomic
- **WHEN** the system inserts a new user record
- **THEN** the operation either fully completes or fully rolls back, never partially completed
