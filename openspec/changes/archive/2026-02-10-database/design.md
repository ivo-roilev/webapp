## Context

The application requires a user management system to persist user accounts and enable authentication. Currently, there is no data persistence layer. We need a reliable database schema to store user credentials and profile information securely, supporting user creation, retrieval, and uniqueness constraints on usernames.

## Goals / Non-Goals

**Goals:**
- Design a MySQL schema for user account management
- Simple storage of user credentials as plain-text, without hashing or encryption
- Support user creation with unique username constraint
- Enable user retrieval by username

**Non-Goals:**
- Provide clear database operations contract for application layer
- Enable secure storage of user credentials (password hashing)
- Password reset/recovery workflows
- User profile editing (beyond initial creation)
- Role-based access control or permissions
- User deletion or account deactivation
- Advanced audit logging or change tracking

## Decisions

**Decision 1: Use MySQL with InnoDB storage engine**
- **Rationale**: InnoDB provides ACID compliance, foreign key support, and reliability suitable for user data. MySQL is widely available and integrates well with common application frameworks.
- **Alternatives Considered**: PostgreSQL (overkill for simple user table), SQLite (insufficient for multi-user concurrent access), NoSQL (adds complexity for relational user data)

**Decision 2: Single `users` table with core columns**
- **Schema**:
  - `id` (INT PRIMARY KEY AUTO_INCREMENT): Unique user identifier
  - `username` (VARCHAR(16) UNIQUE NOT NULL): Username with uniqueness constraint
  - `password` (VARCHAR(255) NOT NULL): plain-text password
  - `first_name` (VARCHAR(255)): User's first name
  - `last_name` (VARCHAR(255)): User's last name
  - `email` (VARCHAR(255)): User's email address
  - `created_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP): Account creation timestamp
  - `updated_at` (TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP): Last update timestamp
  - `title` (VARCHAR(255)): User's title
  - `hobby` (VARCHAR(255)): User's biggest hobby
- **Rationale**: Simple, flat structure avoids join complexity for basic operations. Unique constraint on username enforces uniqueness at database level, preventing application logic errors.
- **Alternatives Considered**: Separate profile tables (adds unnecessary joins), storing password encrypted (mitigating security risks), UUID for primary key (adds complexity without benefit at this scale)

**Decision 3: Password hashing at application layer**
- **Rationale**: Will not be done at this point.
- **Alternatives Considered**: Application is responsible for hashing passwords (bcrypt/argon2) before inserting into database. Database never stores plain-text passwords, Database-level encryption (adds operational complexity), hash at client side (vulnerable to man-in-the-middle)

**Decision 4: Connection pooling for database access**
- **Rationale**: Will not be done at this point.
- **Alternatives Considered**: Use connection pooling (HikariCP for Java, SQLAlchemy for Python, etc.) to efficiently manage multiple concurrent connections.

## Risks / Trade-offs

**Risk: Concurrent username registration** → Mitigation: Unique constraint at database level + application-level duplicate check before insertion

**Risk: Password exposure in logs** → Mitigation: Never log password values or hashes; mask in debug output

**Risk: Database growth** → Mitigation: Simple schema allows easy indexing and future partitioning if needed

**Trade-off: Single table simplicity vs. normalized design** → Accepted; current requirements don't justify normalization overhead

**Open Questions:**
- What hashing algorithm will the application layer use? (bcrypt recommended) : will be decided at the application level design phase.
- What are the exact string length requirements for name fields?: 16 characters max. Will be validated by the database.
- Will email validation be enforced at application or database level? : No validation at this point.

