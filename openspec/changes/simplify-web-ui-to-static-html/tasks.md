## 1. Create Stylesheet

- [x] 1.1 Create src/web/style.css with CSS variables for light/dark mode colors
- [x] 1.2 Add base styles (reset, body, fonts, page layout)
- [x] 1.3 Add theme toggle button styles (top-right corner positioning)
- [x] 1.4 Add form component styles (labels, inputs, buttons, error messages)
- [x] 1.5 Add page/card container styles
- [x] 1.6 Add dark mode overrides with [data-theme="dark"] selector
- [x] 1.7 Add responsive breakpoints for mobile (320px+), tablet (768px+), desktop (1200px+)

## 2. Create HTML Pages

- [x] 2.1 Create src/web/login.html with username/password form
- [x] 2.2 Add form validation and POST request to /api/login in login.html
- [x] 2.3 Add localStorage storage and redirect to user-info.html on success in login.html
- [x] 2.4 Add "Create New User" button linking to create-user.html in login.html
- [x] 2.5 Create src/web/create-user.html with all user fields (username, password, first name, last name, email, title, hobby)
- [x] 2.6 Add form validation (required: username/password) and POST request to /api/users in create-user.html
- [x] 2.7 Add "Back to Login" link in create-user.html
- [x] 2.8 Create src/web/user-info.html with user_id check and redirect logic
- [x] 2.9 Add GET request to /api/users/{user_id} to fetch greeting in user-info.html
- [x] 2.10 Add theme toggle button (🌙/☀️) to all three HTML pages

## 3. Add Theme Toggle Functionality

- [x] 3.1 Add inline JavaScript to detect system theme preference (prefers-color-scheme)
- [x] 3.2 Add JavaScript to load theme from localStorage on page load
- [x] 3.3 Add click handler to toggle between light/dark themes and update localStorage
- [x] 3.4 Ensure theme toggle works consistently across all three pages

## 4. Configure Web Server

- [x] 4.1 Configure web server to serve static files from src/web/ directory
- [x] 4.2 Set login.html as the default/index page
- [x] 4.3 Verify CORS headers allow API calls to backend (http://localhost:8080)
- [x] 4.4 Document API base URL configuration for deployment

## 5. Test Complete Flows

- [ ] 5.1 Test login flow with valid/invalid credentials and error handling
- [ ] 5.2 Test user creation flow with all fields and required-only fields
- [ ] 5.3 Test user-info display and auto-redirect when no session
- [ ] 5.4 Test theme toggle persistence across page reloads on all pages
- [ ] 5.5 Test responsive design on mobile (320px), tablet (768px), desktop (1200px+)
- [ ] 5.6 Test on Chrome 90+, Safari 14+, Firefox 88+
- [ ] 5.7 Verify no console errors and API calls complete successfully

## 6. Cleanup and Documentation

- [ ] 6.1 Remove React/TypeScript files (src/web/src/, package.json, tsconfig*, vite.config.ts, node_modules/)
- [ ] 6.2 Verify src/web/ contains only 5 files (login.html, create-user.html, user-info.html, style.css, index.html redirect)
- [ ] 6.3 Document browser requirements (Chrome 90+, Safari 14+, Firefox 88+) in README
- [ ] 6.4 Document local development setup and API configuration
- [ ] 6.5 Commit implementation and create change summary
