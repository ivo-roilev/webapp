## ADDED Requirements

### Requirement: Support for flexible user metadata
The system SHALL allow storing arbitrary user properties that are not part of the standard profile. These properties MUST be stored in a dedicated `user_metadata` table.

#### Scenario: Storing a new metadata property
- **WHEN** a user property is provided that is not a core field (e.g., "hobby")
- **THEN** the system persists it in the `user_metadata` table with its property name and value

### Requirement: Key-Value storage with parent grouping
The system SHALL support grouping metadata properties using an optional `parent_property` field.

#### Scenario: Storing metadata with a parent group
- **WHEN** a metadata property is saved with a parent property (e.g., parent="preferences", property="theme", value="dark")
- **THEN** it is correctly stored and can be retrieved using both the parent and property keys
