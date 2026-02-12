## Context

The application currently connects to MySQL as the `root` user (configured in `.env`). This is insecure because:
- The app has unnecessary privileges (CREATE DATABASE, DROP, CREATE USER, access to all databases)
- If compromised via SQL injection or other vulnerabilities, an attacker gains full MySQL system access
- Violates the principle of least privilege

The application's actual needs are limited to:
- SELECT, INSERT, UPDATE, DELETE on the `users` table in `webapp_db`
- No DDL operations at runtime (schema is managed separately)

## Goals / Non-Goals

**Goals:**
- Create a dedicated MySQL user (`webapp_user`) with minimal required privileges
- Limit application access to `webapp_db` database only
- Document the separation between setup-time (root) and runtime (webapp_user) operations

**Non-Goals:**
- Automated migration (this is a one-time dev setup change)
- Production secrets management (using plain dev passwords in `.env` for now)
- Connection pooling or advanced MySQL features
- Column-level privilege restrictions (table-level is sufficient)
- Maintain backward compatibility with existing database schema

## Decisions

### Decision 1: Update Existing Schema File vs. Create New Setup File

**Choice:** Update `01_users_schema.sql` to include user creation at the top

**Rationale:**
- Keeps all database setup in one place
- Natural ordering: create user → create database → create tables
- Simpler dev workflow (one file to run)
- `CREATE USER IF NOT EXISTS` makes it idempotent

**Alternative Considered:**
- Separate `00_setup_user.sql` file
- Rejected: Adds complexity with multiple files to track, doesn't provide clear benefit for this single-database app

### Decision 2: Privilege Granularity

**Choice:** Grant SELECT, INSERT, UPDATE, DELETE on `webapp_db.*` (all tables in database)

**Rationale:**
- App needs these four operations for user management
- Database-level grant is simpler than per-table grants
- Future-proof: automatically applies to new tables added to `webapp_db`
- Explicitly excludes dangerous privileges:
  - ❌ CREATE, ALTER, DROP (schema changes)
  - ❌ GRANT OPTION (privilege escalation)
  - ❌ FILE, PROCESS, RELOAD (system operations)

**Alternative Considered:**
- Table-level grants (`GRANT ... ON webapp_db.users`)
- Rejected: Too restrictive if we add more tables, no security benefit for single-database app

### Decision 3: Password Management

**Choice:** Use plain password in `.env` for dev environment (`webapp_dev_password`)

**Rationale:**
- This is a test/dev project
- `.env` should be in `.gitignore` (already standard practice)
- Sufficient security for local development
- Production would use secrets manager (out of scope)

**Alternative Considered:**
- Password hashing, MySQL auth plugins
- Rejected: Overkill for dev environment, adds complexity

### Decision 4: User Scope (localhost vs %)

**Choice:** Create user as `'webapp_user'@'%'` (any host)

**Rationale:**
- Allows the database to run on another server

**Alternative Considered:**
- `'webapp_user'@'localhost'` (local host only)
- Rejected: Unnecessarily restrictive

### Decision 5: Testing Approach (Positive vs Negative Tests)

**Choice:** Split test suite into SQL file for positive tests and shell script for negative tests

**Rationale:**
- SQL scripts fail on first error, cannot assert on expected failures
- Negative tests (privilege denials, access violations) need to verify errors occur correctly
- Shell scripts can check exit codes and grep error messages
- Pattern similar to MySQL's MTR framework but without the heavy Perl/directory structure overhead
- Clear separation: `04_test_suite.sql` for "things that should work", `05_security_tests.sh` for "things that should fail"

**Implementation:**
- Keep `04_test_suite.sql` for positive tests (CRUD operations, schema verification, constraints)
- Create `05_security_tests.sh` for negative security tests:
  - Privilege boundary verification (tasks 4.5-4.7)
  - Duplicate username constraint testing (migrate Test 1.2 from SQL)
  - Access denial assertions using exit codes and grep
- Shell script exits non-zero on test failure for CI integration

**Alternative Considered:**
- MySQL Test Runner (MTR) framework
- Rejected: Heavy setup (Perl, specific directory structure), overkill outside MySQL core development
- Python test script
- Rejected: Additional dependency, more setup than needed for 3-4 security tests
- SQL stored procedures with error handlers
- Rejected: Verbose, awkward syntax, harder to maintain

**Trade-offs:**
- Adds shell scripting to codebase (but simple, standard patterns)
- Tests split across two files (but clear conceptual separation)
- Benefits: Proper negative test assertions, CI-ready, no external dependencies

## Risks / Trade-offs

**Risk:** Developers forget to run updated schema file and still have root user in `.env`
- **Mitigation:** Update README.md with clear setup instructions, add note about security rationale

**Risk:** Existing dev environments have old schema without webapp_user
- **Mitigation:** Use `CREATE USER IF NOT EXISTS` for idempotent setup, document one-time migration step in README

**Trade-off:** Schema changes now require switching to root user
- **Impact:** When adding tables/columns, must use root credentials
- **Acceptable:** Schema changes are infrequent, separation of concerns is worth the extra step

**Trade-off:** Updates needed in three places (.env, schema file, README)
- **Impact:** Small maintenance overhead
- **Acceptable:** One-time change, well-documented

## Migration Plan

### For New Developers:
1. Run updated `src/database/01_users_schema.sql` as root
2. Use `.env` with `DATABASE_USER=webapp_user`
3. Start app normally

### For Existing Developers:
1. Run updated `src/database/01_users_schema.sql` as root
   - Creates webapp_user if it doesn't exist (idempotent)
   - No changes to existing schema/data
2. Update `.env` file:
   ```diff
   -DATABASE_USER=root
   -DATABASE_PASSWORD=
   +DATABASE_USER=webapp_user
   +DATABASE_PASSWORD=webapp_dev_password
   ```
3. Restart app

### Rollback Strategy:
If issues arise, revert `.env` to use root:
```bash
DATABASE_USER=root
DATABASE_PASSWORD=
```

No database changes needed for rollback (webapp_user can remain, unused).

## Open Questions

None - design is straightforward and well-understood from exploration phase.
