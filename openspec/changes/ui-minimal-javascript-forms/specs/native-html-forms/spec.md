# Spec: Native HTML Forms

## ADDED Requirements

### Requirement: Login Form SHALL Submit via Native HTML POST

The login form SHALL use native HTML form submission with method="POST" and action="/api/login", without JavaScript event handlers.

#### Scenario: Login form has correct attributes
- **WHEN** login HTML is parsed
- **THEN** form element has `method="POST"` attribute
- **AND** form element has `action="/api/login"` attribute

#### Scenario: Login form submits without JavaScript
- **WHEN** user submits login form
- **THEN** browser performs native POST request to `/api/login`
- **AND** browser automatically follows 303 redirect from server

#### Scenario: No JavaScript preventDefault on login form
- **WHEN** login HTML JavaScript is examined
- **THEN** no `event.preventDefault()` calls exist for form submission
- **AND** no `fetch()` calls to `/api/login` exist

### Requirement: Create User Form SHALL Submit via Native HTML POST

The create user form SHALL use native HTML form submission with method="POST" and action="/api/create-user", without JavaScript event handlers.

#### Scenario: Create user form has correct attributes
- **WHEN** create-user HTML is parsed
- **THEN** form element has `method="POST"` attribute
- **AND** form element has `action="/api/create-user"` attribute

#### Scenario: Create user form submits without JavaScript
- **WHEN** user submits create user form
- **THEN** browser performs native POST request to `/api/create-user`
- **AND** browser automatically follows 303 redirect from server

#### Scenario: No JavaScript preventDefault on create user form
- **WHEN** create-user HTML JavaScript is examined
- **THEN** no `event.preventDefault()` calls exist for form submission
- **AND** no `fetch()` calls to `/api/create-user` exist

### Requirement: Forms SHALL Use HTML5 Validation Only

Forms SHALL rely on HTML5 `required` attributes and native browser validation without additional JavaScript validation.

#### Scenario: Required fields have HTML5 required attribute
- **WHEN** form HTML is parsed
- **THEN** username and password input fields have `required` attribute

#### Scenario: No JavaScript validation functions exist
- **WHEN** HTML JavaScript is examined
- **THEN** no JavaScript functions for field validation exist
- **AND** no JavaScript validation error messages exist

#### Scenario: Browser performs native validation
- **WHEN** user submits form with empty required field
- **THEN** browser shows native validation message
- **AND** form does not submit until field is filled

### Requirement: All fetch() API Calls for Form Submission SHALL Be Removed

All JavaScript fetch() calls to login and create-user endpoints SHALL be removed from HTML files.

#### Scenario: No fetch to login endpoint
- **WHEN** login HTML JavaScript is examined
- **THEN** no `fetch('/api/login')` or `fetch('http://localhost:8080/api/login')` calls exist

#### Scenario: No fetch to create-user endpoint
- **WHEN** create-user HTML JavaScript is examined
- **THEN** no `fetch('/api/create-user')` or `fetch('http://localhost:8080/api/create-user')` calls exist

### Requirement: All JSON Parsing for Form Responses SHALL Be Removed

All JavaScript code that parses JSON responses from login and create-user endpoints SHALL be removed.

#### Scenario: No JSON parsing after form submission
- **WHEN** login HTML JavaScript is examined
- **THEN** no `response.json()` calls exist after form submission
- **AND** no JSON parsing of user_id or error fields exists

#### Scenario: No manual redirect after form submission
- **WHEN** create-user HTML JavaScript is examined
- **THEN** no `window.location.href` assignments after form submission exist
- **AND** browser handles redirect automatically via HTTP 303

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no existing requirements are being removed.
