## Why

The application has a MySQL user management database schema but lacks a service layer to expose its functionality to clients. Users cannot interact with the database without a RESTful API. This change provides a Rust-based RESTful service that bridges the gap between client applications and the database, enabling user registration, authentication, and profile retrieval through well-defined HTTP endpoints.

## What Changes

- Introduces a new RESTful service written in Rust that connects to the existing MySQL `users` table
- Exposes three primary user management endpoints: user creation, user authentication (login), and user information retrieval
- Provides structured JSON request/response contracts for all endpoints
- Enables applications to interact with the user database without direct database access

## Capabilities

### New Capabilities

- `create-user-endpoint`: HTTP POST endpoint that accepts user registration data (username, password, optional first name, last name, email, title, hobby) and creates a new user record in the database
- `login-endpoint`: HTTP POST endpoint that accepts username and password credentials, validates them against the database, and returns the user ID on successful authentication
- `get-user-info-endpoint`: HTTP GET endpoint that accepts a user ID parameter and returns the user's complete profile information as a formatted string
- `mysql-database-integration`: Database connection layer in Rust that handles connections to the MySQL `users` table, executes queries for user creation, authentication, and retrieval

### Modified Capabilities

(None - this is a new service with no requirement changes to existing capabilities)

## Impact

- **New Services**: Introduces a new Rust service that can be deployed independently or alongside existing application services
- **Database**: Read and write access to the existing MySQL `users` table (no schema changes required)
- **Client Applications**: Any client can now interact with user management features through HTTP endpoints instead of direct database queries
- **Dependencies**: Will require Rust runtime, MySQL client libraries, and HTTP server framework (to be determined in design phase)
