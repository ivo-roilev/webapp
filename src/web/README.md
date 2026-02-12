# Static Web UI - User Management

A simple static HTML/CSS/JavaScript web interface for the User Management REST API. No build tools, no frameworks - just plain web technologies.

## Pages

- **Login** (`login.html`) — Authenticate with username and password (default landing page)
- **Create User** (`create-user.html`) — Register a new user with username, password, and optional profile fields
- **User Info** (`user-info.html`) — View greeting message for the currently authenticated user

## Prerequisites

- Modern web browser (Chrome 90+, Safari 14+, Firefox 88+)
- The Rust REST service running on `http://localhost:8080`
- A simple web server to serve static files (see instructions below)

## Local Development

### Option 1: Python (Built-in)
```bash
cd src/web
python3 -m http.server 8080
```

Then open: `http://localhost:8080`

### Option 2: Node.js (http-server)
```bash
cd src/web
npx http-server -p 8080 --cors
```

Then open: `http://localhost:8080`

### Option 3: PHP (Built-in)
```bash
cd src/web
php -S localhost:8080
```

Then open: `http://localhost:8080`

## API Configuration

The application is configured to make API calls to `http://localhost:8080` by default. This is hardcoded in the JavaScript sections of each HTML file.

To change the API base URL for deployment:
1. Search for `http://localhost:8080` in each HTML file
2. Replace with your production API URL
3. Ensure CORS headers are properly configured on your API server

## Features

- **Responsive Design**: Works on mobile (320px+), tablet (768px+), and desktop (1200px+)
- **Dark Mode**: Theme toggle button on all pages with localStorage persistence
- **System Theme Detection**: Automatically detects user's system preference (prefers-color-scheme)
- **Form Validation**: Client-side validation for required fields
- **Error Handling**: Clear error messages for API failures
- **Session Management**: Uses localStorage for user session persistence

## File Structure

```
src/web/
  index.html         # Redirect to login.html
  login.html         # Login page
  create-user.html   # User creation form
  user-info.html     # User greeting display
  style.css          # Shared stylesheet
```

## Browser Requirements

- **Chrome**: 90+
- **Safari**: 14+
- **Firefox**: 88+

These versions support:
- CSS custom properties (variables)
- Fetch API
- localStorage
- matchMedia (prefers-color-scheme)

## Deployment

1. Copy all `.html` files and `style.css` to your web server
2. Configure web server to serve `index.html` as default (or redirect to `login.html`)
3. Update API base URL in each HTML file if needed
4. Ensure proper CORS headers on your API server

## Testing

### Login Flow
1. Open login.html
2. Enter credentials and submit
3. On success, redirects to user-info.html with user_id in localStorage

### Create User Flow
1. Click "Create New User" from login page
2. Fill in username and password (required)
3. Optionally fill other fields
4. On success, redirects to user-info.html with user_id in localStorage

### User Info Flow
1. Page checks for user_id in localStorage
2. If missing, auto-redirects to login.html
3. If present, fetches greeting from API
4. Displays greeting message

### Theme Toggle
1. Click moon/sun button in top-right corner
2. Theme switches between light and dark
3. Preference saved to localStorage
4. Persists across page reloads

