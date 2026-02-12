## Overview

A simple HTML page providing user login functionality. Users can authenticate with username and password, and upon successful login, their user_id is stored in browser localStorage for session tracking.

## User Interaction

### Login Flow
1. User loads login.html (the default/index page)
2. User enters username and password into an HTML form
3. User clicks "Login" button
4. Form validates that both fields are non-empty
5. If valid, client submits POST request to `/api/login` with username and password
6. Server responds with `{user_id: <number>}`
7. Client stores `user_id` in localStorage
8. Client redirects to user-info.html to display user greeting

### Error Handling
- Display field-level validation errors for empty username/password
- Display API-level error messages (e.g., "Invalid credentials") below the form
- Allow user to retry after error without losing entered data (except password for security)

### Navigation
- Link to "Create New User" page (create-user.html) visible on login page

## Technical Requirements

### Form Fields
- **Username**: Text input, required
- **Password**: Password input (masked), required
- **Error Messages**: Display below each field or as a general alert

### API Integration
- Endpoint: `POST /api/login`
- Request body: `{username: string, password: string}`
- Response: `{user_id: number}`
- Store user_id in `localStorage.setItem('userId', user_id)`
- Redirect to `user-info.html` on success

### UI/UX
- Professional, responsive design (mobile-friendly)
- Clear form labels and instructions
- "Login" button with loading state indicator
- Error messages in red, easily visible
- Minimum browser: Chrome 90+, Safari 14+, Firefox 88+

### No Requirements
- Social login (Google, GitHub, etc.)
- Multi-factor authentication
- Rate limiting (handled by backend)
- Password reset functionality
- Session timeout or automatic logout
