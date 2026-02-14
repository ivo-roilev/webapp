# Spec: Minimal JavaScript Display

## ADDED Requirements

### Requirement: User Info Page SHALL Read user_id from URL Query Parameters

The user-info.html page SHALL read the user_id from URL query parameters instead of localStorage.

#### Scenario: Parse user_id from URL on page load
- **WHEN** user-info page loads with URL `/user-info.html?user_id=123`
- **THEN** JavaScript extracts `user_id=123` from query string using URLSearchParams

#### Scenario: Redirect to login if user_id missing
- **WHEN** user-info page loads without user_id query parameter
- **THEN** page redirects to login page (login.html or index.html)

#### Scenario: Redirect to login if user_id invalid
- **WHEN** user-info page loads with non-numeric user_id
- **THEN** page redirects to login page

### Requirement: User Info Page SHALL Fetch Greeting via GET Request

The user-info.html page SHALL use fetch() to GET user greeting from `/api/users/{user_id}` endpoint.

#### Scenario: Fetch user greeting successfully
- **WHEN** user_id is valid
- **THEN** JavaScript calls `fetch(`/api/users/${user_id}`)`
- **AND** response is parsed as plain text (not JSON)

#### Scenario: Display greeting in DOM
- **WHEN** fetch returns successfully with greeting text
- **THEN** greeting text is displayed in element with id "greetingText" or similar
- **AND** loading message is hidden

#### Scenario: Handle fetch error gracefully
- **WHEN** fetch fails or returns error status
- **THEN** error message is displayed to user
- **AND** loading message is hidden

### Requirement: User Info JavaScript SHALL Be Maximum 20 Lines

The total JavaScript code in user-info.html (excluding comments) SHALL be no more than 20 lines.

#### Scenario: JavaScript code is concise
- **WHEN** user-info.html JavaScript is counted (excluding blank lines and comments)
- **THEN** total lines of code is 20 or fewer

### Requirement: No localStorage SHALL Be Used for user_id

The user-info page SHALL NOT read or write user_id to localStorage.

#### Scenario: No localStorage read for user_id
- **WHEN** user-info page JavaScript is examined
- **THEN** no `localStorage.getItem('user_id')` calls exist

#### Scenario: No localStorage write for user_id
- **WHEN** user-info page JavaScript is examined
- **THEN** no `localStorage.setItem('user_id', ...)` calls exist

### Requirement: No JavaScript SHALL Exist in Login or Create User Pages

The login.html and create-user.html pages SHALL contain zero JavaScript code (excluding theme-related code removal).

#### Scenario: Login page has no JavaScript blocks
- **WHEN** login HTML is parsed
- **THEN** no `<script>` tags with JavaScript code exist
- **AND** no inline event handlers (onclick, onsubmit) exist

#### Scenario: Create user page has no JavaScript blocks
- **WHEN** create-user HTML is parsed
- **THEN** no `<script>` tags with JavaScript code exist
- **AND** no inline event handlers (onclick, onsubmit) exist

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no existing requirements are being removed.
