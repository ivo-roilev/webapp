## 1. Database Schema Refactoring

- [ ] 1.1 Rewrite [src/database/01_users_schema.sql](src/database/01_users_schema.sql) with the three-table structure: `users`, `user_profiles`, and `user_metadata`.
- [ ] 1.2 Include foreign key constraints on `user_id` in `user_profiles` and `user_metadata`.

## 2. Rust Domain Models

- [ ] 2.1 Refactor the `User` struct in [src/rust/main.rs](src/rust/main.rs) to include `Profile` and `Metadata` fields.
- [ ] 2.2 Update `CreateUserPayload` to support the new metadata structure.

## 3. Database Layer Logic

- [ ] 3.1 Implement a multi-stage transaction in `src/rust/db.rs` for `create_user` that inserts into all three tables.
- [ ] 3.2 Optimize `authenticate_user` in `src/rust/db.rs` to only select from the minimized `users` table.
- [ ] 3.3 Update `get_user_by_id` in `src/rust/db.rs` to perform a `LEFT JOIN` on `user_profiles` and query `user_metadata`.
- [ ] 3.4 Ensure the database connection pool is correctly shared across these operations.

## 4. API Handler Integration

- [ ] 4.1 Update the `POST /api/users` handler in [src/rust/main.rs](src/rust/main.rs) to handle profile and metadata mapping.
- [ ] 4.2 Update the `GET /api/users/{user_id}` handler to correctly serialize the aggregated profile and metadata response.

## 5. Validation and Testing

- [ ] 5.1 Update unit tests in [src/rust/tests/main_test.rs](src/rust/tests/main_test.rs) to verify the new table split.
- [ ] 5.2 Verify that the `hobby` and `title` fields are correctly stored in `user_metadata`.
- [ ] 5.3 Run `cargo test` to ensure all 14 tests pass with the new schema.
