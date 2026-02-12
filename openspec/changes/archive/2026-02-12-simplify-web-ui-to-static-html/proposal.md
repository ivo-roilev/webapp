## Why

The existing web application uses TypeScript, React, Vite, and complex component hierarchies, resulting in 10,000+ build artifact files and unnecessary complexity for what are essentially three simple static forms. The application is difficult to modify, maintain, and understand for non-JavaScript developers. A simpler approach using static HTML pages with vanilla JavaScript would be more maintainable and lightweight while preserving all functionality.

## What Changes

Replace the complex TypeScript/React single-page application with three simple static HTML pages:
- Remove Vite build system, TypeScript compilation, and npm dependency chain
- Replace React components (`CreateUserPage`, `LoginPage`, `UserInfoPage`) with plain HTML forms
- Replace React Router navigation with simple HTML links and page redirects
- Replace React state management with browser localStorage
- Keep vanilla JavaScript inline for minimal API interactions
- Remove all links to "Home" navigation from all pages

## Capabilities

### New Capabilities
- `static-create-user-page`: Simple HTML form for creating users with username, password, first name, last name, email, title, and hobby fields. Submits POST request to `/api/users` and stores user_id in localStorage.
- `static-login-page`: Simple HTML form for user login with username and password. Submits POST request to `/api/login` and stores user_id in localStorage. Have this page as the index, the default page displayed. Have the ability to either login an existing user (`login` button), or redirect to the `static-create-user-page` (`Create New User` button).
- `static-user-info-page`: Simple HTML page displaying user greeting fetched from `/api/users/{user_id}`. Retrieves user_id from localStorage.
- `static-web-styling`: Responsive CSS stylesheet for all three pages with professional design, navigation bar, and footer.

### Modified Capabilities
- None

## Impact

- **Web layer**: Reduced from 10,000+ files to 4 HTML files + 1 CSS file
- **Build process**: Eliminated - files can be served directly without compilation
- **Dependencies**: Removed - no npm, Node.js, Vite, React, React Router, or TypeScript needed
- **File size**: Dramatically reduced - single static files instead of bundled assets
- **Maintainability**: Increased - all code is plain HTML/CSS/JavaScript, easy to read and modify
- **Functionality preserved**: All three UIs work identically to the React version
