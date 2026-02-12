# Static User Info Page Specification

## Overview
A simple HTML page that displays a personalized greeting for authenticated users by retrieving user information from the backend.

## File Location
`src/web/user-info.html`

## Functional Requirements

### Session Management
- Check for `user_id` in `localStorage` on page load
- If no `user_id` found:
  - Redirect immediately to `login.html`
  - Display no content

### Greeting Display
- Fetch user greeting from backend using stored `user_id`
- Display greeting text in prominent card
- Show loading state while fetching
- Handle fetch errors gracefully

### User Actions
1. **View Greeting**: Automatic on page load
2. **Retry on Error**: Button to retry failed fetch
3. **Toggle Theme**: Switch between light and dark modes

### API Integration
- **Endpoint**: `GET http://localhost:8080/api/users/{user_id}`
- **Method**: GET (not POST)
- **Response format**: Plain text (not JSON)
- **Response example**: "Hello, John! Welcome back."
- **Error handling**:
  - Display error message
  - Provide "Retry" button to attempt fetch again

### Navigation
- No navigation links (user must use browser back button or manually navigate)
- No logout button per requirements
- No home or navigation bar links

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
    <title>User Information</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <!-- Theme toggle button -->
    <!-- Greeting display card -->
    <!-- Error handling section (hidden by default) -->
    <!-- Inline JavaScript for session check, theme, and greeting fetch -->
  </body>
</html>
```

### JavaScript Functionality
- Session validation on load
- GET request to fetch greeting (not POST)
- Response parsing as text (not JSON)
- Error handling with retry mechanism
- Theme toggle with system preference detection

### Greeting Fetch Logic
```javascript
async function fetchUserGreeting() {
  try {
    const response = await fetch(
      `http://localhost:8080/api/users/${userId}`,
      { method: 'GET' }
    );

    if (!response.ok) {
      throw new Error('Failed to fetch user information');
    }

    // Parse as TEXT not JSON
    const greeting = await response.text();
    // Display greeting
  } catch (error) {
    // Show error with retry button
  }
}
```

### Page Title Styling
- Title "User Information" uses distinct styling:
  - Smaller font size than greeting
  - Uppercase with letter-spacing
  - Visually separated from greeting text

### Accessibility
- Semantic HTML structure
- Clear visual hierarchy
- Error messages announced properly
- Keyboard navigation support
- Focus states for interactive elements

### Browser Support
- Chrome 90+
- Safari 14+
- Firefox 88+

## Success Criteria
- Page redirects to login.html if no user_id in localStorage
- Greeting fetches and displays correctly for valid user_id
- GET request used (not POST)
- Response parsed as text (not JSON)
- Error state shows "Retry" button
- Retry successfully fetches greeting after error
- No logout button present per requirements
- Theme toggle persists across page reloads
- Page title styled distinctly from greeting text
- No framework dependencies (vanilla HTML/CSS/JS)
