## 1. Database Schema Refactoring

- [x] 1.1 Rewrite [src/database/01_users_schema.sql](src/database/01_users_schema.sql) with the three-table structure: `users`, `user_profiles`, and `user_metadata`.
- [x] 1.2 Include foreign key constraints on `user_id` in `user_profiles` and `user_metadata`.

## 2. Rust Domain Models

- [x] 2.1 Refactor the `User` struct in [src/rust/main.rs](src/rust/main.rs) to include `Profile` and `Metadata` fields.
- [x] 2.2 Update `CreateUserPayload` to support the new metadata structure.

## 3. Database Layer Logic

- [x] 3.1 Implement a multi-stage transaction in `src/rust/db.rs` for `create_user` that inserts into all three tables.
- [x] 3.2 Optimize `authenticate_user` in `src/rust/db.rs` to only select from the minimized `users` table.
- [x] 3.3 Update `get_user_by_id` in `src/rust/db.rs` to perform a `LEFT JOIN` on `user_profiles` and query `user_metadata`.
- [x] 3.4 Ensure the database connection pool is correctly shared across these operations.

## 4. API Handler Integration

- [x] 4.1 Update the `POST /api/users` handler in [src/rust/main.rs](src/rust/main.rs) to handle profile and metadata mapping.
- [x] 4.2 Update the `GET /api/users/{user_id}` handler to correctly serialize the aggregated profile and metadata response.

## 5. Validation and Testing

- [x] 5.1 Update unit tests in [src/rust/tests/main_test.rs](src/rust/tests/main_test.rs) to verify the new table split.
- [x] 5.2 Verify that the `hobby` and `title` fields are correctly stored in `user_metadata`.
- [x] 5.3 Run `cargo test` to ensure all tests pass with the new schema.
- [x] 5.4 Confirm that the EAV pattern in `user_metadata` correctly handles arbitrary user data.
