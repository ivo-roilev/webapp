# Static Login Page Specification

## Overview
A simple HTML login page that serves as the application's entry point, allowing users to authenticate or navigate to user creation.

## File Location
`src/web/login.html` (also served as `src/web/index.html` via redirect)

## Functional Requirements

### Form Fields
- **Username field**:
  - Input type: `text`
  - Required: Yes
  - ID: `username`
  - Placeholder text provided

- **Password field**:
  - Input type: `password`
  - Required: Yes
  - ID: `password`
  - Placeholder text provided

### User Actions
1. **Login**: Submit credentials to authenticate
2. **Create New User**: Navigate to user creation page
3. **Toggle Theme**: Switch between light and dark modes

### API Integration
- **Endpoint**: `POST http://localhost:8080/api/login`
- **Request body**: JSON with `username` and `password`
- **Success response**: JSON with `user_id` field
- **Success action**:
  - Store `user_id` in `localStorage`
  - Redirect to `user-info.html`
- **Error handling**: Display error message in red text below form

### Navigation
- Link to create-user.html for new user registration
- No home or navigation bar links per requirements

### Theme Support
- Theme toggle button (🌙/☀️ emoji) in top-right corner
- Persist theme preference in `localStorage` as `theme` key
- Detect system preference on first load via `prefers-color-scheme`
- Apply theme via `data-theme` attribute on document element

## Technical Requirements

### HTML Structure
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <!-- Theme toggle button -->
    <!-- Login form with username/password -->
    <!-- Link to create-user.html -->
    <!-- Inline JavaScript for theme and form handling -->
  </body>
</html>
```

### JavaScript Functionality
- Form submission handler with validation
- Fetch API call to backend
- localStorage operations for session and theme
- Error display logic
- Theme toggle with system preference detection

### Accessibility
- Semantic HTML with proper labels
- Form validation messages
- Keyboard navigation support
- Focus states for interactive elements

### Browser Support
- Chrome 90+
- Safari 14+
- Firefox 88+

## Success Criteria
- Valid credentials authenticate successfully and redirect to user-info.html
- Invalid credentials show error message
- "Create New User" link navigates to create-user.html
- Theme toggle persists across page reloads
- Form validates required fields before submission
- No framework dependencies (vanilla HTML/CSS/JS)
