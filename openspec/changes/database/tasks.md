## 1. Database Setup

- [ ] 1.1 Create MySQL database schema with users table
- [ ] 1.2 Set up database connection with connection pooling
- [ ] 1.3 Create database migration/initialization script

## 2. User Creation Feature

- [ ] 2.1 Implement user creation validation (required fields, username length)
- [ ] 2.2 Implement username uniqueness check before insertion
- [ ] 2.3 Implement database insert operation for new user
- [ ] 2.4 Handle duplicate username error responses
- [ ] 2.5 Return created user record with all fields

## 3. User Retrieval Feature

- [ ] 3.1 Implement user retrieval by username query
- [ ] 3.2 Implement "user not found" error handling
- [ ] 3.3 Return complete user record (id, username, password, name fields, email, title, hobby, timestamps)
- [ ] 3.4 Validate retrieval input (empty/invalid username handling)

## 4. Testing & Validation

- [ ] 4.1 Test user creation with valid data
- [ ] 4.2 Test user creation with duplicate username
- [ ] 4.3 Test user creation with missing fields
- [ ] 4.4 Test user creation with oversized username (>16 chars)
- [ ] 4.5 Test user retrieval with existing username
- [ ] 4.6 Test user retrieval with non-existent username
- [ ] 4.7 Test user retrieval with invalid input
- [ ] 4.8 Verify unique constraint enforcement at database level
