## User Management Database Implementation

### Location
All database files are located in `src/database/`

### Database Security: Dedicated User Setup

**Rationale:** The application uses a dedicated MySQL user (`webapp_user`) with minimal privileges following the principle of least privilege. This limits the security impact if the application is compromised.

**Separation of Concerns:**
- **Root user**: Used only for schema setup and migrations (DDL operations: CREATE, ALTER, DROP)
- **webapp_user**: Used by the application at runtime (DML operations only: SELECT, INSERT, UPDATE, DELETE)

**Privileges:**
- `webapp_user` has SELECT, INSERT, UPDATE, DELETE privileges on `webapp_db` database only
- `webapp_user` **cannot** perform schema changes (CREATE TABLE, ALTER TABLE, DROP)
- `webapp_user` **cannot** access MySQL system tables or other databases
- `webapp_user` **cannot** create other users or escalate privileges

### Setup Instructions

#### For New Developers

1. **Run the schema setup script as MySQL root:**
   ```bash
   mysql -u root -p < src/database/01_users_schema.sql
   ```
   This creates:
   - The `webapp_user` with host pattern `%` (any host)
   - Grants required privileges
   - Creates the `webapp_db` database
   - Creates the `users` table with indexes

2. **Configure application to use dedicated user:**
   - The `.env` file is already configured with `DATABASE_USER=webapp_user`
   - Password is set to `webapp_dev_password` (dev environment only)

3. **Start the application:**
   ```bash
   cargo run
   ```

#### For Existing Developers (Migration)

If you previously had the database set up with root user:

1. **Run the updated schema script as root (idempotent):**
   ```bash
   mysql -u root -p < src/database/01_users_schema.sql
   ```
   - This creates `webapp_user` if it doesn't exist (no-op if already exists)
   - No changes made to existing data or schema

2. **Update your `.env` file:**
   ```dotenv
   DATABASE_USER=webapp_user
   DATABASE_PASSWORD=webapp_dev_password
   ```

3. **Restart the application**

#### Schema Changes

**Important:** When making schema changes (adding tables, columns, indexes):
- Use MySQL root credentials
- Update `src/database/01_users_schema.sql`
- Re-run as root: `mysql -u root -p < src/database/01_users_schema.sql`
- The `webapp_user` privileges automatically apply to new tables in `webapp_db`

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

**3. 05_security_tests.sh**
- Security and privilege boundary tests for `webapp_user`:
  - ✅ Verify `webapp_user` cannot CREATE TABLE (DDL blocked)
  - ✅ Verify `webapp_user` cannot query `mysql.user` system table
  - ✅ Verify `webapp_user` cannot CREATE USER (privilege escalation blocked)
  - ✅ Verify `webapp_user` cannot DROP TABLE (destructive DDL blocked)
  - ✅ Verify `webapp_user` can SELECT from `webapp_db.users` (read access works)
  - ✅ Verify duplicate username rejection works for `webapp_user`
- Exit code 0 = all tests passed, non-zero = failures detected
- Run with: `./src/database/05_security_tests.sh`

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
# Positive functional tests (requires webapp_db to exist)
mysql webapp_db < src/database/04_test_suite.sql

# Security and privilege boundary tests (requires webapp_user to exist)
./src/database/05_security_tests.sh
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
