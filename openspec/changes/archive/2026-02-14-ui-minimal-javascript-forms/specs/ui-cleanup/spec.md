# Spec: UI Cleanup and Simplification

## ADDED Requirements

### Requirement: User Info Page SHALL NOT Have Redundant Heading

The user-info.html page SHALL NOT contain an `<h1>User Information</h1>` heading as it provides no value to users viewing their personalized greeting.

#### Scenario: No H1 heading in user info page
- **WHEN** user-info.html is parsed
- **THEN** no `<h1>` element with text "User Information" exists
- **AND** personalized greeting is the primary content

#### Scenario: Page title in browser tab identifies page
- **WHEN** user views user-info page
- **THEN** browser tab shows meaningful title (e.g., "User Info - User Management")
- **AND** no redundant heading duplicates this information in page body

### Requirement: Semantic Structure SHALL Be Maintained

While removing redundant headings, the overall semantic HTML structure SHALL be maintained for accessibility.

#### Scenario: Main content remains in semantic containers
- **WHEN** user-info.html is parsed
- **THEN** greeting content is within appropriate semantic elements (div, main, section)
- **AND** card or container structure is preserved

#### Scenario: Accessibility is not compromised
- **WHEN** screen reader reads user-info page
- **THEN** page structure is understandable
- **AND** main content is easily discoverable

### Requirement: Entry Point SHALL Be login.html Renamed to index.html

The login.html file SHALL be renamed to index.html to serve as the application entry point, eliminating the need for a separate redirect page.

#### Scenario: Login page is accessible as index.html
- **WHEN** user navigates to application root
- **THEN** index.html serves the login form directly
- **AND** no redirect occurs

#### Scenario: Old index.html redirect is removed
- **WHEN** web directory is examined
- **THEN** no separate redirect-only index.html exists
- **AND** login page serves as the primary entry point

#### Scenario: References to login.html are updated
- **WHEN** other HTML files are examined
- **THEN** links to "login.html" are updated to "index.html"
- **AND** JavaScript redirects to login are updated to redirect to "index.html"

### Requirement: Other Page Headings SHALL Be Reviewed for Value

Other pages (login.html, create-user.html) SHALL be reviewed to determine if their headings provide value or should be removed.

#### Scenario: Login page heading is evaluated
- **WHEN** login page is examined
- **THEN** decision is made whether heading adds value or is redundant

#### Scenario: Create user page heading is evaluated
- **WHEN** create-user page is examined
- **THEN** decision is made whether heading adds value or is redundant

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no existing requirements are being removed.
