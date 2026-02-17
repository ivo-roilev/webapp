# Static Web UI - User Management

A simple static HTML/CSS/JavaScript web interface for the User Management REST API. No build tools, no frameworks - just plain web technologies.

## Pages

- **Login** (`index.html`) — Authenticate with username and password (default landing page)
- **Create User** (`create-user.html`) — Register a new user with username, password, and optional profile fields
- **User Info** (`user-info.html`) — View greeting message for the currently authenticated user

## Prerequisites

- Modern web browser (Chrome 90+, Safari 14+, Firefox 88+)
- The Rust REST service running (see Local Development below)

## Local Development

Start the application with a single command:

```bash
cargo run
```

This starts the Actix-web server on `http://localhost:8080` which serves both:
- REST API endpoints (`/api/*`)
- Static HTML/CSS files (`/`, `*.html`, `*.css`)

Then open: `http://localhost:8080`

**Note:** Static files are now served directly by the Rust server. No separate web server is needed.

## Features

- **Single Server**: One process serves both API and static files
- **Responsive Design**: Works on mobile (320px+), tablet (768px+), and desktop (1200px+)
- **Form Validation**: Client-side validation for required fields
- **Error Handling**: Clear error messages for API failures

## File Structure

```
src/web/
  index.html         # Login page (default)
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

The static files are served by the Actix-web server from the `src/web` directory:

1. Ensure all `.html` files and `style.css` are in `src/web/`
2. Run `cargo build --release` to build the optimized binary
3. Deploy the binary and ensure `src/web/` is accessible relative to the binary
4. The server will automatically serve static files from `./src/web`

## Testing

### Login Flow
1. Open http://localhost:8080
2. Enter credentials and submit
3. On success, redirects to user-info.html with user_id in query parameter

### Create User Flow
1. Click "Create New User" from login page
2. Fill in username and password (required)
3. Optionally fill other fields
4. On success, redirects to user-info.html with user_id in query parameter

### User Info Flow
1. Page checks for user_id in URL query parameter
2. If missing, auto-redirects to index.html
3. If present, fetches greeting from API
4. Displays greeting message

