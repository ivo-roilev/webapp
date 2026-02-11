# Capability: UI User Info

## Purpose
Web page for displaying user profile information retrieved from the REST API.

## Requirements

### Requirement: User Information Display
The system SHALL retrieve and display user information on the user-info page based on the user_id.

#### Scenario: Page loads with user ID from previous login
- **WHEN** user navigates to the user-info page after successful login or user creation
- AND system sends GET request to `/api/users/{user_id}` with the provided user_id

#### Scenario: User data displayed successfully
- **WHEN** system receives successful response from `/api/users/{user_id}`
- **THEN** all received response text is displayed to the user in the center of the page.

#### Scenario: Missing or invalid user ID
- **WHEN** user_id is invalid
- **THEN** system displays a message redirecting user to login or create user page
- AND the page does not attempt to fetch user data

### Requirement: User Information Presentation
The system SHALL present user data in a clear, organized format suitable for display.

### Requirement: Error Handling for User Retrieval
The system SHALL handle errors gracefully when retrieving user information.

#### Scenario: User not found
- **WHEN** system receives a 404 or "not found" response from `/api/users/{user_id}`
- **THEN** an error message is displayed indicating the user could not be found
- AND user is offered an option to return to login or create user page

#### Scenario: Server error during retrieval
- **WHEN** system receives a server error response from `/api/users/{user_id}`
- **THEN** an error message is displayed to the user
- AND user is offered an option to retry loading the page
