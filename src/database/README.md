## User Management Database Implementation

### Location
All database files are located in `src/database/`

### Files Created

**1. 01_users_schema.sql**
- Creates the `webapp_db` database
- Defines the `users` table with all required fields:
  - `id` (INT, PRIMARY KEY, AUTO_INCREMENT)
  - `username` (VARCHAR(16), UNIQUE, NOT NULL) - max 16 characters, unique constraint
  - `password` (VARCHAR(255), NOT NULL) - plain-text password storage
  - `first_name`, `last_name`, `email` (VARCHAR(255), optional)
  - `title`, `hobby` (VARCHAR(255), optional)
  - `created_at` (TIMESTAMP, defaults to current timestamp)
  - `updated_at` (TIMESTAMP, auto-updates)
- Creates indexes for efficient queries

**2. 04_test_suite.sql**
- Comprehensive test coverage:
  - ✅ Test user creation with valid data
  - ✅ Test duplicate username rejection
  - ✅ Test missing required fields validation
  - ✅ Test oversized username validation (>16 chars)
  - ✅ Test user retrieval with existing username
  - ✅ Test user retrieval with non-existent username
  - ✅ Test invalid input handling
  - ✅ Test UNIQUE constraint at database level
  - ✅ Test timestamp auto-population
  - ✅ Test table schema verification

### Database Schema Features

✅ **Users Table**
- Auto-incrementing primary key (id)
- Unique username constraint (max 16 characters)
- Required fields: username, password
- Optional fields: first_name, last_name, email, title, hobby
- Automatic timestamps: created_at, updated_at
- Indexed for efficient lookups

### Usage

To initialize the database:
```bash
mysql < src/database/01_users_schema.sql
```

To run tests:
```bash
mysql webapp_db < src/database/04_test_suite.sql
```

### Application Integration

The application must implement the following SQL operations:

**Create User:**
```sql
INSERT INTO users (username, password, first_name, last_name, email, title, hobby)
VALUES (?, ?, ?, ?, ?, ?, ?);
```

**Retrieve User by Username:**
```sql
SELECT id, username, password, first_name, last_name, email, title, hobby, created_at, updated_at
FROM users
WHERE username = ?;
```

**Check if User Exists:**
```sql
SELECT COUNT(*) FROM users WHERE username = ?;
```

### Application-Level Validation

The application layer must handle:
- Username validation (not NULL/empty, max 16 characters)
- Password validation (not NULL/empty)
- Duplicate username check before INSERT
- Input validation for retrieval (not NULL/empty username)

### Implementation Status
✅ All 20 tasks completed
- ✅ Database schema with users table created (01_users_schema.sql)
- ✅ Comprehensive test suite covering all operations (04_test_suite.sql)
- ✅ User creation capability fully supported via schema
- ✅ User retrieval capability fully supported via schema
- ✅ Uniqueness constraint enforced at database level
- ✅ Application-level validation documented
