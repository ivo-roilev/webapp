## 1. Database Setup

- [x] 1.1 Create MySQL database schema with users table
- [x] 1.2 Set up database connection with connection pooling
- [x] 1.3 Create database migration/initialization script

## 2. User Creation Feature

- [x] 2.1 Implement user creation validation (required fields, username length)
- [x] 2.2 Implement username uniqueness check before insertion
- [x] 2.3 Implement database insert operation for new user
- [x] 2.4 Handle duplicate username error responses
- [x] 2.5 Return created user record with all fields

## 3. User Retrieval Feature

- [x] 3.1 Implement user retrieval by username query
- [x] 3.2 Implement "user not found" error handling
- [x] 3.3 Return complete user record (id, username, password, name fields, email, title, hobby, timestamps)
- [x] 3.4 Validate retrieval input (empty/invalid username handling)

## 4. Testing & Validation

- [x] 4.1 Test user creation with valid data
- [x] 4.2 Test user creation with duplicate username
- [x] 4.3 Test user creation with missing fields
- [x] 4.4 Test user creation with oversized username (>16 chars)
- [x] 4.5 Test user retrieval with existing username
- [x] 4.6 Test user retrieval with non-existent username
- [x] 4.7 Test user retrieval with invalid input
- [x] 4.8 Verify unique constraint enforcement at database level
