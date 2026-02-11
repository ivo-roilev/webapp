# Capability: UI Create User

## Purpose
Web form interface for creating new users via the REST API.

## Requirements

### Requirement: User Creation Form Display
The system SHALL display a web form with all user fields: username, password, first_name, last_name, email, title, and hobby. The username and password are required. The rest of the fields are optional.

#### Scenario: Form loads on page initialization
- **WHEN** user navigates to the create user page
- **THEN** the form is displayed with all input fields empty and ready for input

#### Scenario: Form validation feedback
- **WHEN** user attempts to submit the form with missing required fields - username and password
- **THEN** the system displays validation errors indicating which fields are missing

### Requirement: User Creation API Request
The system SHALL send a POST request to `/api/users` with all user data when the user clicks the "create user" button.

#### Scenario: Successful user creation
- **WHEN** user fills all form fields with valid data and clicks "create user"
- **THEN** system sends POST request to `/api/users` with username, password, first_name, last_name, email, title, and hobby
- AND system receives `user_id` in the response
- AND user is redirected to the `UI-user-info` page with the returned `user_id`

#### Scenario: Server error during creation
- **WHEN** system receives an error response from `/api/users` endpoint
- **THEN** an error message is displayed to the user
- AND user remains on the create user page
- AND user_id is not stored

### Requirement: Form Input Validation
The system SHALL validate user input before submission to catch obvious errors early.

#### Scenario: Validate username
- **WHEN** user does not enter data in the username field
- **THEN** the system provides immediate feedback (visual or via validation message)

#### Scenario: Validate password
- **WHEN** user does not enter data in the password field
- **THEN** the system provides immediate feedback (visual or via validation message)
