## 1. Update Database Setup Script

- [x] 1.1 Add CREATE USER statement at top of `src/database/01_users_schema.sql` with `IF NOT EXISTS` clause for `webapp_user@%`
- [x] 1.2 Set password for `webapp_user` to `webapp_dev_password` in CREATE USER statement
- [x] 1.3 ADD GRANT statement to give SELECT, INSERT, UPDATE, DELETE privileges on `webapp_db.*` to `webapp_user@%`
- [x] 1.4 Add FLUSH PRIVILEGES statement after GRANT
- [x] 1.5 Ensure user creation and grants execute before database and table creation

## 2. Update Configuration

- [x] 2.1 Change `DATABASE_USER=root` to `DATABASE_USER=webapp_user` in `.env`
- [x] 2.2 Change `DATABASE_PASSWORD=` to `DATABASE_PASSWORD=webapp_dev_password` in `.env`

## 3. Update Documentation

- [x] 3.1 Update `src/database/README.md` to document the dedicated user approach
- [x] 3.2 Add section explaining separation of concerns (root for schema, webapp_user for runtime)
- [x] 3.3 Document setup instructions for new developers (run script as root)
- [x] 3.4 Document migration instructions for existing developers
- [x] 3.5 Add note about security rationale (principle of least privilege)
- [x] 3.6 Document that schema changes require root credentials

## 4. Verification and Testing

- [x] 4.1 Test database setup script creates `webapp_user` successfully on fresh MySQL
- [x] 4.2 Test database setup script is idempotent (can run multiple times)
- [x] 4.3 Verify application connects successfully using `webapp_user` credentials
- [x] 4.4 Test application can perform data operations (POST /api/users, GET /api/users/{id})
- [x] 4.5 Create `src/database/05_security_tests.sh` shell script for negative security tests
- [x] 4.6 Add test: Verify `webapp_user` cannot execute CREATE TABLE command (check exit code and "Access denied" error)
- [x] 4.7 Add test: Verify `webapp_user` cannot query `mysql.user` system table (check "Access denied")
- [x] 4.8 Add test: Verify `webapp_user` cannot execute CREATE USER command (check "Access denied")
- [x] 4.9 Move Test 1.2 (duplicate username rejection) from `04_test_suite.sql` to `05_security_tests.sh`
- [x] 4.10 Update `src/database/README.md` to document both test files (`04_test_suite.sql` for positive tests, `05_security_tests.sh` for negative security tests)
- [x] 4.11 Make `05_security_tests.sh` executable and verify it exits non-zero on test failures
- [x] 4.12 Run `05_security_tests.sh` to confirm all security tests pass with restricted webapp_user
- [x] 4.13 Confirm application endpoints work correctly with new credentials
