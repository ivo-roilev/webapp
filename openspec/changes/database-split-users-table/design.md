## Context
The current database schema stores all user information in a single table. This includes authentication data (username, password) and profile details (names, email, hobbies). This design is inefficient for authentication-only queries and makes it difficult to manage optional or varied user attributes.

## Goals / Non-Goals

**Goals:**
- Normalize the database by splitting the `users` table into three: `users` (auth), `user_profiles` (standard info), and `user_metadata` (flexible info).
- Improve query performance for login by reducing the row size of the authentication table.
- Provide a scalable way to store arbitrary user properties without schema changes (EAV model).
- Ensure data integrity across multiple tables using SQL transactions in the Rust backend.

**Non-Goals:**
- Changing primary keys or basic user identification logic.
- Implementing password hashing (maintaining existing plain-text requirement for now).
- Updating the web UI beyond ensuring it continues to work with the refactored backend.

## Decisions

1. **Normalized Table Structure**:
    - `users`: (id, username, password) - Primary source of truth for identity.
    - `user_profiles`: (user_id, first_name, last_name, email) - Standard profile fields.
    - `user_metadata`: (user_id, parent_property, property, value) - Key-Value storage for dynamic fields like `hobby` and `title`.
    - *Rationale*: Clear separation of concerns and optimized for the most frequent operation (login).

2. **Backend Model Composition**:
    - The Rust `User` struct will be updated to include nested structures for profile and metadata.
    - *Rationale*: Reflects the relational structure while maintaining a coherent domain model.

3. **Transaction-based Persistence**:
    - User creation will use `BEGIN ... COMMIT` to ensure `users`, `user_profiles`, and `user_metadata` are updated atomically.
    - *Rationale*: Prevents orphaned auth records if profile creation fails.

4. **Query Strategy**:
    - Retrieval will use `LEFT JOIN` between `users` and `user_profiles`, plus a separate query or join for `user_metadata`.
    - *Rationale*: Efficiently gathers fragmented data for full profile views.

## Risks / Trade-offs

- **Risk**: Increased query complexity for full user retrieval.
- **Mitigation**: Use SQL joins and ensure `user_id` is indexed in all dependent tables.
- **Risk**: Potential performance hit for writes due to multiple inserts.
- **Mitigation**: Use single-connection transactions to minimize overhead.
- **Risk**: Breaking change for existing data.
- **Mitigation**: Provide a comprehensive SQL migration script to transform the data structure.
