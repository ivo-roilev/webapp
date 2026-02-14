# Spec: URL State Management

## ADDED Requirements

### Requirement: User ID SHALL Be Passed via URL Query Parameters

After successful login or user creation, the user_id SHALL be passed to the user-info page via URL query parameter instead of localStorage.

#### Scenario: Login redirect includes user_id in URL
- **WHEN** backend redirects after successful login
- **THEN** redirect URL is `/user-info.html?user_id={id}`

#### Scenario: Create user redirect includes user_id in URL
- **WHEN** backend redirects after successful user creation
- **THEN** redirect URL is `/user-info.html?user_id={id}`

#### Scenario: User info page receives user_id from URL
- **WHEN** user-info page loads after redirect
- **THEN** user_id is available in window.location.search
- **AND** can be extracted using URLSearchParams

### Requirement: No localStorage SHALL Be Used for State Management

No page SHALL use localStorage to store or retrieve user_id or any user session state.

#### Scenario: Login page does not store user_id in localStorage
- **WHEN** login JavaScript is examined
- **THEN** no `localStorage.setItem('user_id', ...)` calls exist

#### Scenario: Create user page does not store user_id in localStorage
- **WHEN** create-user JavaScript is examined
- **THEN** no `localStorage.setItem('user_id', ...)` calls exist

#### Scenario: User info page does not read user_id from localStorage
- **WHEN** user-info JavaScript is examined
- **THEN** no `localStorage.getItem('user_id')` calls exist

#### Scenario: No other localStorage usage for state
- **WHEN** all HTML files are examined
- **THEN** no localStorage calls exist except potentially for theme (which is being removed separately)

### Requirement: Browser History SHALL Work Correctly with URL Parameters

Users SHALL be able to use browser back/forward buttons and the URL with query parameters SHALL remain functional.

#### Scenario: User can bookmark user-info page
- **WHEN** user is on `/user-info.html?user_id=123`
- **THEN** page can be bookmarked
- **AND** bookmark reopens to same user's info

#### Scenario: Browser back works after redirect
- **WHEN** user is redirected to user-info page after login
- **AND** user clicks browser back button
- **THEN** user returns to login page
- **AND** no redirect loop occurs

#### Scenario: Direct URL access works
- **WHEN** user directly navigates to `/user-info.html?user_id=123`
- **THEN** page loads user information correctly

## MODIFIED Requirements

None - these are new requirements.

## REMOVED Requirements

None - no existing requirements are being removed.
