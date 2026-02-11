## MODIFIED Requirements

### Requirement: User Information Display
The system SHALL retrieve and display user information on the user-info page based on the user_id, handling plain text response from the API.

#### Scenario: Page loads with user ID from previous login
- **WHEN** user navigates to the user-info page after successful login or user creation
- **THEN** system sends GET request to `/api/users/{user_id}` with the provided user_id
- AND expects `Content-Type: text/plain` response

#### Scenario: User data displayed successfully
- **WHEN** system receives HTTP 200 response from `/api/users/{user_id}` with plain text content
- **THEN** the received text (e.g., `"Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"`) is displayed to the user in the center of the page
- AND no JSON parsing is performed

#### Scenario: Minimal user data displayed
- **WHEN** system receives HTTP 200 response containing minimal greeting text (e.g., `"Hello jdoe, welcome!"`)
- **THEN** the greeting text is displayed in the center of the page

#### Scenario: Missing or invalid user ID
- **WHEN** user_id is invalid or missing
- **THEN** system displays a message redirecting user to login or create user page
- AND the page does not attempt to fetch user data

### Requirement: User Information Presentation
The system SHALL present user data as plain text exactly as received from the API, without additional formatting or parsing.

#### Scenario: Text rendered as-is
- **WHEN** plain text response is received from `/api/users/{user_id}`
- **THEN** the system displays the text without attempting to parse fields or apply custom styling
- AND the text is shown in a readable, centered format

### Requirement: Error Handling for User Retrieval
The system SHALL handle errors gracefully when retrieving user information, displaying plain text error messages from the API.

#### Scenario: User not found
- **WHEN** system receives HTTP 404 response from `/api/users/{user_id}` with plain text error message
- **THEN** the error message text (e.g., `"User with ID 42 not found"`) is displayed to the user
- AND user is offered an option to return to login or create user page

#### Scenario: Invalid user ID format
- **WHEN** system receives HTTP 400 response from `/api/users/{user_id}` with plain text error message
- **THEN** the error message text (e.g., `"user_id must be a valid integer"`) is displayed to the user
- AND user is offered an option to return to login or create user page

#### Scenario: Server error during retrieval
- **WHEN** system receives HTTP 500 or 503 response from `/api/users/{user_id}` with plain text error message
- **THEN** the error message text is displayed to the user
- AND user is offered an option to retry loading the page
