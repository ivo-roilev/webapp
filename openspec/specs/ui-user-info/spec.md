# Capability: UI User Info

## Purpose
Web page for displaying user profile information as a plain text greeting retrieved from the REST API.

## Requirements

### Requirement: Text Greeting Display
The system SHALL retrieve and display the user greeting text on the user-info page.

#### Scenario: Page loads with user ID from previous login
- **WHEN** user navigates to the user-info page after successful login or user creation
- AND system sends GET request to `/api/users/{user_id}` with the provided user_id
- **THEN** the system expects a plain text response (Content-Type: text/plain)

#### Scenario: Greeting message displayed successfully
- **WHEN** system receives successful response with greeting text from `/api/users/{user_id}`
- **THEN** the greeting text is displayed prominently and centered on the page
- **EXAMPLES:**
  - Full greeting: "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
  - Minimal greeting: "Hello jdoe, welcome!"

#### Scenario: Missing or invalid user ID
- **WHEN** user_id is invalid or missing
- **THEN** system displays a message redirecting user to login or create user page
- AND the page does not attempt to fetch user data

### Requirement: Plain Text Response Handling
The system SHALL process plain text responses from the API instead of JSON.

#### Scenario: API response is plain text
- **WHEN** system receives response from `/api/users/{user_id}` with Content-Type: text/plain
- **THEN** the response body is displayed directly without JSON parsing

#### Scenario: Response rendering
- **WHEN** greeting text is received
- **THEN** the text is displayed in centered, readable format on the page

### Requirement: Error Handling for User Retrieval
The system SHALL handle errors gracefully when retrieving user information.

#### Scenario: User not found (404)
- **WHEN** system receives a 404 error with plain text "User with ID {user_id} not found"
- **THEN** an error message is displayed indicating the user could not be found
- AND user is offered an option to return to login or create user page

#### Scenario: Invalid user ID format (400)
- **WHEN** system receives a 400 error with plain text "user_id must be a valid integer"
- **THEN** an error message is displayed to the user
- AND user is offered options to navigate back

#### Scenario: Server error during retrieval (500, 503)
- **WHEN** system receives a 500 or 503 error response from `/api/users/{user_id}`
- **THEN** plain text error message is displayed to the user
- AND user is offered an option to retry loading the page or return to login
