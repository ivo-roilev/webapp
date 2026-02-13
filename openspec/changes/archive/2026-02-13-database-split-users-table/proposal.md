## Why
The current `users` table is overloaded, combining authentication credentials with optional profile information. This leads to inefficient queries during login (where only `username` and `password` are needed) and poor data normalization for optional fields. Splitting the table into specialized structures will improve performance and provide a more flexible way to store arbitrary user properties.

## What Changes
- **Database Schema**: Transition from a single `users` table to three distinct tables:
    - `users`: Core authentication data (**BREAKING** - schema change).
        - `id` (PK)
        - `username`
        - `password`
    - `user_profiles`: Standard profile information linked to `users`.
        - `user_id` (FK to `users.id`)
        - `first_name`
        - `last_name`
        - `email`
    - `user_metadata`: Flexible key-value storage for additional info.
        - `user_id` (FK to `users.id`)
        - `parent_property` (optional)
        - `property`
        - `value`
- **Rust Backend**:
    - Update `User` and related structs in [src/rust/main.rs](src/rust/main.rs) to match the new relational model.
    - Refactor [src/rust/db.rs](src/rust/db.rs) to handle transactions across multiple tables for user creation.
    - Optimize the login query to only target the `users` table.
    - Update user retrieval to perform necessary joins across `users`, `user_profiles`, and `user_metadata`.

## Capabilities

### New Capabilities
- `flexible-user-metadata`: Support for arbitrary, non-fixed user properties like hobbies and titles via the `user_metadata` table.

### Modified Capabilities
- `user-creation`: Requirement to persist data across the new three-table structure atomically.
- `user-retrieval`: Requirement to aggregate data from multiple tables to provide a complete user profile.
- `login-endpoint`: Requirement to only query the core authentication table for credentials.

## Impact
- **Database**: [src/database/01_users_schema.sql](src/database/01_users_schema.sql) needs to be completely rewritten.
- **Data Layer**: [src/rust/db.rs](src/rust/db.rs) requires significant changes to query logic and transactions.
- **Application Models**: [src/rust/main.rs](src/rust/main.rs) needs new struct definitions and handler updates.
- **REST API**: Response format for Get User Info remains the same, but internal retrieval logic changes.
