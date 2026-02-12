## Overview

A simple HTML page displaying personalized user information/greeting. After login or user creation, this page retrieves the user's greeting text from the backend and displays it. The page retrieves the user_id from browser localStorage to identify the user.

## User Interaction

### Display Flow
1. User arrives at user-info.html (either redirected after login or after account creation)
2. Page checks if user_id exists in localStorage
3. If user_id missing: display message "No user session found" with automatically redirects to the main login page
4. If user_id exists: page shows loading indicator while fetching data
5. Page submits POST request to `/api/users/{user_id}` to fetch user greeting
6. Server responds with greeting text (e.g., "Hello John! You are a Software Engineer who enjoys photography")
7. Page displays the greeting in a prominent, readable format

### Error Handling
- If API call fails: display error message and provide "Retry" button
- Allow user to retry fetch without losing session
- No links from this page!

### Navigation
- "Logout" button clears localStorage and returns to login/home
- "Retry" button on error allows refetching user info

## Technical Requirements

### Session Management
- Retrieve user_id from `localStorage.getItem('userId')`
- If user_id is null/undefined: display "No user session found" with navigation links
- On logout: call `localStorage.removeItem('userId')` and redirect to login/home

### API Integration
- Endpoint: `POST /api/users/{user_id}`
- Request body: `{}` (empty JSON object)
- Request header: `Content-Type: application/json`
- Response: `{greeting: string}`
- Display greeting as-is in the page

### UI/UX
- Large, readable display of greeting text with pleasant formatting
- Error state with "Retry" button and links
- Professional, responsive design (mobile-friendly)
- Loading state indicator while fetching data
- Minimum browser: Chrome 90+, Safari 14+, Firefox 88+

### States

#### Loading
- Show "Loading..." text or spinner
- Disable retry button until response received

#### Success
- Display greeting prominently
- Show "Logout" button and retry button

#### Error
- Display error message
- Show "Retry" button
- Show links to login/create-user pages

#### No Session
- Display "No user session found" message
- Redirects to login page

### No Requirements
- User profile editing
- Avatar/profile pictures
- User preferences or settings
- Session timeout warnings
- Activity tracking or analytics
- Navigation away from this page
