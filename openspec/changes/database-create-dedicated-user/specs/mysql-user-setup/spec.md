## ADDED Requirements

### Requirement: Dedicated MySQL user exists
The system SHALL create a MySQL user named `webapp_user` for application runtime database access. The user creation MUST be idempotent using `CREATE USER IF NOT EXISTS`.

#### Scenario: User creation on fresh database
- **WHEN** database setup script is executed on a fresh MySQL instance
- **THEN** MySQL user `webapp_user@%` is created successfully

#### Scenario: User creation on existing database
- **WHEN** database setup script is executed where `webapp_user` already exists
- **THEN** script completes without error and preserves existing user

### Requirement: User has minimal required privileges
The `webapp_user` SHALL be granted only SELECT, INSERT, UPDATE, and DELETE privileges on the `webapp_db` database. The user MUST NOT have DDL privileges (CREATE, ALTER, DROP) or administrative privileges (GRANT, CREATE USER, FILE, PROCESS, RELOAD).

#### Scenario: User can perform data operations
- **WHEN** application connects as `webapp_user`
- **THEN** application can execute SELECT, INSERT, UPDATE, DELETE queries on `webapp_db.users` table

#### Scenario: User cannot perform schema changes
- **WHEN** application attempts to execute CREATE TABLE as `webapp_user`
- **THEN** MySQL rejects the query with insufficient privileges error

#### Scenario: User cannot access system tables
- **WHEN** application attempts to query `mysql.user` table as `webapp_user`
- **THEN** MySQL rejects the query with access denied error

#### Scenario: User cannot create other users
- **WHEN** application attempts to execute CREATE USER as `webapp_user`
- **THEN** MySQL rejects the query with insufficient privileges error

### Requirement: User has database-level scope
The `webapp_user` SHALL have host pattern `%` to allow connections from any host. Privileges SHALL be scoped to `webapp_db.*` (all tables in webapp_db database) and MUST NOT extend to other databases.

#### Scenario: User can connect from remote host
- **WHEN** application running on a different host connects to MySQL
- **THEN** connection is accepted with `webapp_user` credentials

#### Scenario: User cannot access other databases
- **WHEN** application attempts to query tables in a different database (e.g., `mysql.user`)
- **THEN** MySQL rejects the query with access denied error

### Requirement: Application uses dedicated user
The application's `.env` configuration file SHALL specify `DATABASE_USER=webapp_user` and include the corresponding password. The application MUST connect to MySQL using these credentials at runtime.

#### Scenario: Application connects with webapp_user
- **WHEN** application starts and reads `.env` configuration
- **THEN** application establishes MySQL connection as `webapp_user` (not root)

#### Scenario: Application performs normal operations
- **WHEN** user creates an account via POST /api/users
- **THEN** application successfully inserts user record using `webapp_user` credentials

### Requirement: Setup script provides user and privileges
The database setup script `src/database/01_users_schema.sql` SHALL include SQL statements to create `webapp_user` and grant required privileges. The user creation and grants MUST execute before database and table creation statements.

#### Scenario: Setup script creates user before schema
- **WHEN** DBA executes `01_users_schema.sql` as MySQL root user
- **THEN** script creates `webapp_user`, grants privileges, then creates `webapp_db` and `users` table

#### Scenario: Setup script is idempotent
- **WHEN** DBA executes `01_users_schema.sql` multiple times
- **THEN** all executions succeed without errors (user creation uses IF NOT EXISTS)
