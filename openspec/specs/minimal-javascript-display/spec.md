# Spec: Minimal JavaScript Display

## Purpose

This spec defines requirements for minimal JavaScript usage in displaying user information, focusing on URL-based state management and fetch API for data retrieval.

## Requirements

### Requirement: User Info Page SHALL Read user_id from URL Query Parameters

The user-info.html page SHALL read the user_id from URL query parameters instead of localStorage.

#### Scenario: Parse user_id from URL on page load
- **WHEN** user-info page loads with URL `/user-info.html?user_id=123`
- **THEN** JavaScript extracts `user_id=123` from query string using URLSearchParams

#### Scenario: Redirect to login if user_id missing
- **WHEN** user-info page loads without user_id query parameter
- **THEN** page redirects to index.html

#### Scenario: Redirect to login if user_id invalid
- **WHEN** user-info page loads with non-numeric user_id
- **THEN** page redirects to index.html

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

### Requirement: Login and Create User Pages SHALL Have Minimal JavaScript

The login.html (index.html) and create-user.html pages SHALL contain minimal JavaScript (~12 lines each) only for form submission handling.

#### Scenario: Login page has minimal JavaScript for form submission
- **WHEN** index.html JavaScript is examined
- **THEN** JavaScript code is approximately 12 lines
- **AND** JavaScript only handles form submission, response parsing, and redirect
- **AND** no validation, theme, or other logic exists

#### Scenario: Create user page has minimal JavaScript for form submission
- **WHEN** create-user HTML JavaScript is examined
- **THEN** JavaScript code is approximately 12 lines
- **AND** JavaScript only handles form submission, response parsing, and redirect
- **AND** no validation, theme, or other logic exists
