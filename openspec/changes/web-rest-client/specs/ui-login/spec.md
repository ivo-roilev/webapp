## ADDED Requirements

### Requirement: Login Form Display
The system SHALL display a login form with two fields: username and password.

#### Scenario: Login form loads on page initialization
- **WHEN** user navigates to the login page
- **THEN** the login form is displayed with both input fields empty and ready for input

#### Scenario: Form validation feedback
- **WHEN** user attempts to submit the login form with empty fields
- **THEN** the system displays validation errors indicating which fields are missing

### Requirement: Login API Request
The system SHALL send a POST request to `/api/login` with username and password when the user clicks the "login" button.

#### Scenario: Successful login
- **WHEN** user fills both form fields with valid credentials and clicks "login"
- **THEN** system sends POST request to `/api/login` with username and password
- AND system receives `user_id` in the response
- AND user is redirected to the `UI-user-info` page with the returned `user_id`

#### Scenario: Invalid credentials
- **WHEN** system receives an authentication error response from `/api/login` endpoint
- **THEN** an error message is displayed to the user (e.g., "Invalid username or password")
- AND user remains on the login page
- AND user_id is not stored

#### Scenario: Network error during login
- **WHEN** the `/api/login` endpoint is unreachable or returns a server error
- **THEN** an error message is displayed to the user
- AND user remains on the login page
- AND user_id is not stored
