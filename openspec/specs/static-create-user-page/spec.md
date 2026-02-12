## Overview

A simple HTML page providing user creation/registration functionality. Users can create a new account with required fields (username, password) and optional fields (first name, last name, email, title, hobby). Upon successful account creation, the user_id is stored in browser localStorage and the user is taken to the user-info page.

## User Interaction

### Registration Flow
1. User loads create-user.html
2. User enters required fields: username and password
3. User optionally fills in: first name, last name, email, title, hobby
4. User clicks "Create User" button
5. Form validates that username and password are non-empty
6. If valid, client submits POST request to `/api/users` with all provided data
7. Server responds with `{user_id: <number>}`
8. Client stores `user_id` in localStorage
9. Client redirects to user-info.html to display user greeting

### Error Handling
- Display field-level validation errors for empty username/password
- Display API-level error messages (e.g., "Username already exists") below the form
- Allow user to retry after error without losing entered data

### Navigation
- Link to go back to the "Login" page (login.html) visible on create-user page if the user Already have an account

## Technical Requirements

### Form Fields
- **Username** (required): Text input
- **Password** (required): Password input (masked)
- **First Name** (optional): Text input
- **Last Name** (optional): Text input
- **Email** (optional): Email input
- **Title** (optional): Text input
- **Hobby** (optional): Text input
- **Error Messages**: Display below each field or as a general alert

### API Integration
- Endpoint: `POST /api/users`
- Request body: `{username: string, password: string, first_name?: string, last_name?: string, email?: string, title?: string, hobby?: string}`
- Only include optional fields in JSON if they have values
- Response: `{user_id: number}`
- Store user_id in `localStorage.setItem('userId', user_id)`
- Redirect to `user-info.html` on success

### UI/UX
- Professional, responsive design (mobile-friendly)
- Clear form labels distinguishing required vs optional fields
- "Create User" button with loading state indicator
- Error messages in red, easily visible
- Back button to main login page for existing users
- Minimum browser: Chrome 90+, Safari 14+, Firefox 88+

### No Requirements
- Email validation (beyond HTML5 input type)
- Duplicate username detection on client (server handles)
- Password strength requirements on client
- CAPTCHA or bot detection
- Terms of service acceptance
- Email verification
