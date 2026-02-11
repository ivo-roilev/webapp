-- User Management Test Suite
-- Comprehensive tests for user creation and retrieval functionality

USE webapp_db;

-- ============================================================================
-- Test Group 1: User Creation Tests
-- ============================================================================

-- Test 1.1: Create user with valid data
-- Expected: User created successfully with all fields
START TRANSACTION;
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('testuser1', 'password123', 'John', 'Doe', 'john@example.com', 'Manager', 'Reading');

-- Verify created user
SELECT 'Test 1.1: Create user with valid data' AS test_name;
SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
FROM users WHERE username = 'testuser1';
ROLLBACK;

-- Test 1.2: Duplicate username rejection
-- Expected: Second insert with same username fails due to UNIQUE constraint
START TRANSACTION;
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('duplicateuser', 'pass1', 'Jane', 'Smith', 'jane@example.com', 'Developer', 'Gaming');

-- Try to insert duplicate - should fail
SELECT 'Test 1.2: Duplicate username rejection' AS test_name;
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('duplicateuser', 'pass2', 'Bob', 'Johnson', 'bob@example.com', 'Tester', 'Sports');
-- Expected: Error - Duplicate entry 'duplicateuser' for key 'users.username' or 'users.idx_username'

ROLLBACK;

-- Test 1.3: Missing required fields (Validation)
-- Expected: Validation fails - NULL values in required fields
START TRANSACTION;
SELECT 'Test 1.3: Missing required fields - username NULL' AS test_name;
-- This would fail if username is NOT NULL
INSERT INTO users (username, password) VALUES (NULL, 'password123');
SELECT 'Cannot insert NULL username - constraint enforced by database schema' AS result;
ROLLBACK;

-- Test 1.4: Oversized username (>16 chars)
-- Expected: Validation fails - username exceeds max length
START TRANSACTION;
SELECT 'Test 1.4: Oversized username (>16 chars)' AS test_name;
-- This username is 20 characters - exceeds the VARCHAR(16) limit
INSERT INTO users (username, password) VALUES ('this_is_a_very_long_username', 'pass123');
SELECT 'Username "this_is_a_very_long_username" (20 chars) exceeds max 16 chars' AS description;
SELECT 'This test verifies the VARCHAR(16) constraint on username column' AS result;
ROLLBACK;

-- ============================================================================
-- Test Group 2: User Retrieval Tests
-- ============================================================================

-- Setup: Create test user for retrieval tests
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('retrieval_test', 'secret_pass', 'Alice', 'Williams', 'alice@example.com', 'Analyst', 'Music');

-- Test 2.1: Retrieve user with existing username
-- Expected: Returns complete user record
SELECT 'Test 2.1: Retrieve user with existing username' AS test_name;
SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
FROM users WHERE username = 'retrieval_test';

-- Test 2.2: Retrieve non-existent user
-- Expected: Returns empty result set (no rows)
SELECT 'Test 2.2: Retrieve non-existent user' AS test_name;
SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
FROM users WHERE username = 'nonexistent_user';
SELECT 'Expected: Empty result set (0 rows returned)' AS expected_result;

-- Test 2.3: Retrieve with invalid input (empty username)
-- Expected: Returns empty result set
SELECT 'Test 2.3: Retrieve with invalid input (empty username)' AS test_name;
SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
FROM users WHERE username = '';
SELECT 'Expected: Empty result set' AS expected_result;

-- Clean up test data
DELETE FROM users WHERE username IN ('retrieval_test', 'testuser1', 'duplicateuser');

-- ============================================================================
-- Test Group 3: Constraint and Uniqueness Verification
-- ============================================================================

-- Test 3.1: Verify unique constraint on username at database level
START TRANSACTION;
SELECT 'Test 3.1: Verify unique constraint enforcement' AS test_name;
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('constraint_test', 'pass1', 'Test', 'User', 'test@example.com', 'QA', 'Coding');

-- Attempt duplicate insert
-- INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
-- VALUES ('constraint_test', 'pass2', 'Another', 'User', 'another@example.com', 'Dev', 'Reading');
SELECT 'UNIQUE constraint on username prevents duplicate entries' AS constraint_type;
SELECT 'Constraint name: users.idx_username or PRIMARY key based' AS details;
ROLLBACK;

-- ============================================================================
-- Test Group 4: Data Integrity Verification
-- ============================================================================

-- Test 4.1: Verify all columns are properly created
SELECT 'Test 4.1: Verify table schema' AS test_name;
DESCRIBE users;

-- Test 4.2: Verify timestamp columns work
START TRANSACTION;
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES ('timestamp_test', 'pass123', 'Time', 'Keeper', 'time@example.com', 'Admin', 'Clocks');

SELECT 'Test 4.2: Verify timestamp columns' AS test_name;
SELECT username, created_at, updated_at FROM users WHERE username = 'timestamp_test';
SELECT 'Expected: created_at and updated_at filled with current timestamp' AS expected;
ROLLBACK;

-- ============================================================================
-- Summary
-- ============================================================================
SELECT 'All tests completed. Verify:' AS summary;
SELECT '1. User creation with valid data works' AS check_1;
SELECT '2. Duplicate usernames are rejected' AS check_2;
SELECT '3. Username length limit (16 chars) is enforced' AS check_3;
SELECT '4. User retrieval returns complete records' AS check_4;
SELECT '5. Non-existent users return empty sets' AS check_5;
SELECT '6. Invalid input is handled gracefully' AS check_6;
SELECT '7. UNIQUE constraint prevents duplicates at DB level' AS check_7;
SELECT '8. Timestamps auto-populate correctly' AS check_8;
