# Spec: Relative URLs in HTML Forms

## Purpose

This spec defines requirements for updating HTML forms to use relative URLs instead of absolute URLs, enabling the application to work on any port or domain without hardcoded values.

## Requirements

### Requirement: Form Actions SHALL Use Relative Paths

All HTML form action attributes SHALL use relative paths starting with `/` instead of absolute URLs with protocol and host.

#### Scenario: Login form action is relative
- **WHEN** `src/web/index.html` is examined
- **THEN** form element has `action="/api/login"`
- **AND** NOT `action="http://localhost:8080/api/login"`

#### Scenario: Create user form action is relative
- **WHEN** `src/web/create-user.html` is examined
- **THEN** form element has `action="/api/create-user"`
- **AND** NOT `action="http://localhost:8080/api/create-user"`

### Requirement: JavaScript Fetch Calls SHALL Use Relative Paths

JavaScript fetch API calls SHALL use relative paths instead of absolute URLs.

#### Scenario: User info fetch uses relative URL
- **WHEN** `src/web/user-info.html` JavaScript is examined
- **THEN** fetch call uses `/api/users/${userId}`
- **AND** NOT `http://localhost:8080/api/users/${userId}`

#### Scenario: Fetch URL starts with forward slash
- **WHEN** any fetch call is examined
- **THEN** the URL parameter starts with `/`
- **AND** does not include protocol or hostname

### Requirement: Navigation Links SHALL Use Relative Paths

Navigation links between pages SHALL use relative paths or page names without domain.

#### Scenario: Link to index from create-user page
- **WHEN** `src/web/create-user.html` is examined
- **THEN** link to login uses `href="index.html"` or `href="/"`
- **AND** NOT `href="http://localhost:8080/index.html"`

#### Scenario: Link to create-user from index page
- **WHEN** `src/web/index.html` is examined
- **THEN** link to create user uses `href="create-user.html"`
- **AND** NOT `href="http://localhost:8080/create-user.html"`

### Requirement: Relative URLs SHALL Work on Any Port

Relative URLs SHALL function correctly regardless of the port number the server is running on.

#### Scenario: Application works on port 8080
- **WHEN** server runs on `http://localhost:8080`
- **THEN** form submissions work correctly
- **AND** navigation works correctly

#### Scenario: Application works on port 3000
- **WHEN** server runs on `http://localhost:3000`
- **THEN** form submissions work correctly
- **AND** navigation works correctly

#### Scenario: Application works on custom domain
- **WHEN** server runs on `https://example.com`
- **THEN** form submissions work correctly
- **AND** navigation works correctly

### Requirement: No Hardcoded Localhost References

HTML files SHALL NOT contain hardcoded references to `localhost` or `127.0.0.1`.

#### Scenario: No localhost in form actions
- **WHEN** all HTML files are searched for "localhost"
- **THEN** no form action attributes contain "localhost"

#### Scenario: No localhost in JavaScript fetch
- **WHEN** all HTML files are searched for "localhost"
- **THEN** no fetch calls contain "localhost"

#### Scenario: No localhost in links
- **WHEN** all HTML files are searched for "localhost"
- **THEN** no anchor href attributes contain "localhost"

### Requirement: Browser Behavior SHALL Be Unchanged

The user experience and browser behavior SHALL remain identical after switching to relative URLs.

#### Scenario: Form submission works as before
- **WHEN** user submits login form
- **THEN** POST request is sent to the correct API endpoint
- **AND** redirect behavior is unchanged
- **AND** user sees same result as with absolute URLs

#### Scenario: Navigation works as before
- **WHEN** user clicks link to another page
- **THEN** browser navigates to the correct page
- **AND** URL in address bar is correct

#### Scenario: Fetch API works as before
- **WHEN** JavaScript makes fetch request
- **THEN** request is sent to correct API endpoint
- **AND** response is processed correctly
- **AND** data display is unchanged

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no requirements are being removed.