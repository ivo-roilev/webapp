# Static Create User Page Specification

## Overview
A simple HTML form for user registration with 7 input fields (2 required, 5 optional), enabling new users to create accounts.

## File Location
`src/web/create-user.html`

## Functional Requirements

### Form Fields

#### Required Fields
- **Username**:
  - Input type: `text`
  - Required: Yes
  - ID: `username`
  - Validation: Must not be empty

- **Password**:
  - Input type: `password`
  - Required: Yes
  - ID: `password`
  - Validation: Must not be empty

#### Optional Fields
- **First Name**:
  - Input type: `text`
  - ID: `firstName`
  - Optional: Omitted from API request if empty

- **Last Name**:
  - Input type: `text`
  - ID: `lastName`
  - Optional: Omitted from API request if empty

- **Email**:
  - Input type: `email`
  - ID: `email`
  - Optional: Omitted from API request if empty

- **Title**:
  - Input type: `text`
  - ID: `title`
  - Optional: Omitted from API request if empty

- **Hobby**:
  - Input type: `text`
  - ID: `hobby`
  - Optional: Omitted from API request if empty

### User Actions
1. **Create User**: Submit form to create new account
2. **Back to Login**: Navigate back to login page
3. **Toggle Theme**: Switch between light and dark modes

### API Integration
- **Endpoint**: `POST http://localhost:8080/api/users`
- **Request body**: JSON object containing:
  - Always: `username`, `password`
  - Conditionally: `first_name`, `last_name`, `email`, `title`, `hobby` (only if non-empty)
- **Success response**: JSON with `user_id` field
- **Success action**:
  - Store `user_id` in `localStorage`
  - Redirect to `user-info.html`
- **Error handling**: Display error message in red text below form

### Navigation
- Link to login.html to return to authentication
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
    <title>Create New User</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <!-- Theme toggle button -->
    <!-- Create user form with 7 fields -->
    <!-- Link to login.html -->
    <!-- Inline JavaScript for theme and form handling -->
  </body>
</html>
```

### JavaScript Functionality
- Form submission handler with validation
- Conditional object building (omit empty optional fields)
- Fetch API call to backend
- localStorage operations for session and theme
- Error display logic
- Theme toggle with system preference detection

### Data Transformation
```javascript
// Build userData object conditionally
const userData = {
  username: username,
  password: password,
  ...(firstName && { first_name: firstName }),
  ...(lastName && { last_name: lastName }),
  ...(email && { email: email }),
  ...(title && { title: title }),
  ...(hobby && { hobby: hobby })
};
```

### Accessibility
- Semantic HTML with proper labels
- Required field indicators (*)
- Form validation messages
- Keyboard navigation support
- Focus states for interactive elements

### Browser Support
- Chrome 90+
- Safari 14+
- Firefox 88+

## Success Criteria
- User creation succeeds with only username and password
- Optional fields are omitted from request when empty
- User creation succeeds with all 7 fields populated
- Success redirects to user-info.html with stored user_id
- Failure shows appropriate error message
- "Back to Login" link navigates to login.html
- Theme toggle persists across page reloads
- Form validates required fields before submission
- No framework dependencies (vanilla HTML/CSS/JS)
