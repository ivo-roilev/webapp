## Why

The application currently connects to MySQL using the `root` user, which is a security anti-pattern. If the application is compromised (e.g., through SQL injection or other vulnerabilities), an attacker would have full database system access, including the ability to drop databases, create users, and access sensitive system tables. This violates the principle of least privilege.

## What Changes

- Create a new MySQL user `webapp_user` with minimal privileges (SELECT, INSERT, UPDATE, DELETE on `webapp_db.*` only)
- Update the database setup script `src/database/01_users_schema.sql` for creating the dedicated user
- Update `.env` configuration to use `webapp_user` instead of `root`
- Update `src/database/README.md` with setup instructions for the dedicated user workflow
- Document the separation of concerns: root for schema migrations, webapp_user for runtime operations

## Capabilities

### New Capabilities
- `mysql-user-setup`: Database user creation with principle of least privilege for MySQL connections

### Modified Capabilities
- None

## Impact

- **Configuration**: `.env` file (DATABASE_USER and DATABASE_PASSWORD fields)
- **Database scripts**: `src/database/01_users_schema.sql` file updated
- **Documentation**: `src/database/README.md` (updated setup workflow)
- **Security posture**: Limits blast radius of potential SQL injection or application compromise
- **Dev workflow**: One-time setup step to create dedicated user before running app
- **Runtime behavior**: No functional changes to application endpoints or user experience
