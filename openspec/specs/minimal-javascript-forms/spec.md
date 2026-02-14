# Spec: Minimal JavaScript Form Submission

## Purpose

This spec defines requirements for HTML form submission using minimal JavaScript (~12 lines per form) to intercept submission, send form-encoded data, and handle client-side redirects.

## Requirements

### Requirement: Login Form SHALL Use Minimal JavaScript for Submission

The login form SHALL use minimal JavaScript (~12 lines) to intercept form submission, send data via fetch(), and handle the redirect client-side.

#### Scenario: Login form has correct attributes
- **WHEN** login HTML is parsed
- **THEN** form element has `id="loginForm"` attribute
- **AND** form element has `method="POST"` attribute
- **AND** form element has `action="http://localhost:8080/api/login"` attribute

#### Scenario: Login form JavaScript intercepts submission
- **WHEN** user submits login form
- **THEN** JavaScript calls `event.preventDefault()`
- **AND** JavaScript sends fetch() request to `/api/login` with URLSearchParams body
- **AND** JavaScript sets Content-Type to `application/x-www-form-urlencoded`

#### Scenario: Login success triggers client-side redirect
- **WHEN** server returns 200 OK with user_id
- **THEN** JavaScript reads response as plain text
- **AND** JavaScript redirects to `user-info.html?user_id=${userId}`

#### Scenario: Login failure shows error message
- **WHEN** server returns non-OK status
- **THEN** JavaScript displays error message to user
- **AND** no redirect occurs

### Requirement: Create User Form SHALL Use Minimal JavaScript for Submission

The create user form SHALL use minimal JavaScript (~12 lines) to intercept form submission, send data via fetch(), and handle the redirect client-side.

#### Scenario: Create user form has correct attributes
- **WHEN** create-user HTML is parsed
- **THEN** form element has `id="createUserForm"` attribute
- **AND** form element has `method="POST"` attribute
- **AND** form element has `action="http://localhost:8080/api/create-user"` attribute

#### Scenario: Create user form JavaScript intercepts submission
- **WHEN** user submits create user form
- **THEN** JavaScript calls `event.preventDefault()`
- **AND** JavaScript sends fetch() request to `/api/create-user` with URLSearchParams body
- **AND** JavaScript sets Content-Type to `application/x-www-form-urlencoded`

#### Scenario: Create user success triggers client-side redirect
- **WHEN** server returns 200 OK with user_id
- **THEN** JavaScript reads response as plain text
- **AND** JavaScript redirects to `user-info.html?user_id=${userId}`

#### Scenario: Create user failure shows error message
- **WHEN** server returns non-OK status
- **THEN** JavaScript displays error message to user
- **AND** no redirect occurs

### Requirement: Forms SHALL Use HTML5 Validation Only

Forms SHALL rely on HTML5 `required` attributes and native browser validation without additional JavaScript validation logic.

#### Scenario: Required fields have HTML5 required attribute
- **WHEN** form HTML is parsed
- **THEN** username and password input fields have `required` attribute

#### Scenario: No JavaScript validation functions exist
- **WHEN** HTML JavaScript is examined
- **THEN** no JavaScript functions for field validation exist
- **AND** no JavaScript validation error messages exist (except for server errors)

#### Scenario: Browser performs native validation
- **WHEN** user submits form with empty required field
- **THEN** browser shows native validation message
- **AND** form does not submit until field is filled

### Requirement: Form Data SHALL Be Sent as URL-Encoded

Form submissions SHALL convert FormData to URLSearchParams to ensure proper `application/x-www-form-urlencoded` encoding.

#### Scenario: Form data is URL-encoded
- **WHEN** JavaScript submits form
- **THEN** fetch() uses `new URLSearchParams(formData)` as body
- **AND** Content-Type header is set to `application/x-www-form-urlencoded`

#### Scenario: Server receives properly encoded data
- **WHEN** server receives form submission
- **THEN** data is parsed as form-encoded (not multipart or JSON)
- **AND** all field names and values are correctly decoded
