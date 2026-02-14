# Implementation Tasks

## 1. Backend API Changes

- [x] 1.1 Rename `/api/users` route to `/api/create-user` in main.rs
- [x] 1.2 Modify `login()` handler to return 303 redirect with Location header
- [x] 1.3 Modify `create_user()` handler to return 303 redirect with Location header
- [x] 1.4 Update handler tests to expect 303 status and Location header instead of JSON
- [x] 1.5 Verify redirect URLs include `user_id` query parameter in correct format
- [x] 1.6 Run backend tests: `cargo test`

## 2. CSS Theming Updates

- [x] 2.1 Add `@media (prefers-color-scheme: dark)` block to style.css
- [x] 2.2 Move dark theme CSS variables inside dark media query
- [x] 2.3 Add `@media (prefers-color-scheme: light)` block to style.css
- [x] 2.4 Move light theme CSS variables inside light media query
- [x] 2.5 Verify all color variables are defined in both theme blocks

## 3. Remove Theme Toggle from Login Page

- [x] 3.1 Remove theme toggle button element from login.html
- [x] 3.2 Remove all theme toggle JavaScript from login.html
- [x] 3.3 Remove localStorage theme get/set calls from login.html
- [x] 3.4 Remove updateThemeIcon function from login.html
- [x] 3.5 Verify login page has zero theme-related JavaScript

## 4. Remove Theme Toggle from Create User Page

- [x] 4.1 Remove theme toggle button element from create-user.html
- [x] 4.2 Remove all theme toggle JavaScript from create-user.html
- [x] 4.3 Remove localStorage theme get/set calls from create-user.html
- [x] 4.4 Remove updateThemeIcon function from create-user.html
- [x] 4.5 Verify create-user page has zero theme-related JavaScript

##  5. Remove Theme Toggle from User Info Page

- [x] 5.1 Remove theme toggle button element from user-info.html
- [x] 5.2 Remove all theme toggle JavaScript from user-info.html
- [x] 5.3 Remove localStorage theme get/set calls from user-info.html
- [x] 5.4 Remove updateThemeIcon function from user-info.html

## 6. Convert Login to Native Form

- [x] 6.1 Update login form with `method="POST"` attribute
- [x] 6.2 Update login form with `action="/api/login"` attribute
- [x] 6.3 Add `required` attributes to username and password inputs
- [x] 6.4 Remove form submit preventDefault JavaScript
- [x] 6.5 Remove fetch() call to /api/login
- [x] 6.6 Remove JSON parsing and response handling code
- [x] 6.7 Remove manual redirect after login
- [x] 6.8 Remove all client-side validation JavaScript
- [x] 6.9 Verify login.html has zero JavaScript blocks

## 7. Convert Create User to Native Form

- [x] 7.1 Update create user form with `method="POST"` attribute
- [x] 7.2 Update create user form with `action="/api/create-user"` attribute
- [x] 7.3 Add `required` attributes to username and password inputs
- [x] 7.4 Remove form submit preventDefault JavaScript
- [x] 7.5 Remove fetch() call to /api/create-user
- [x] 7.6 Remove JSON parsing and response handling code
- [x] 7.7 Remove manual redirect after user creation
- [x] 7.8 Remove all client-side validation JavaScript
- [x] 7.9 Verify create-user.html has zero JavaScript blocks

## 8. Update User Info Page with Minimal JavaScript

- [x] 8.1 Remove localStorage.getItem('user_id') call
- [x] 8.2 Add URLSearchParams to extract user_id from query string
- [x] 8.3 Add redirect to login if user_id is missing or invalid
- [x] 8.4 Update fetch URL to use user_id from query params
- [x] 8.5 Ensure greeting is displayed from plain text response
- [x] 8.6 Remove all localStorage.setItem calls for user_id
- [x] 8.7 Verify total JavaScript is ≤20 lines (excluding blank lines and comments)

## 9. Remove Redundant Page Heading

- [x] 9.1 Remove `<h1>User Information</h1>` from user-info.html
- [x] 9.2 Verify semantic structure is maintained (div containers preserved)
- [x] 9.3 Verify page title in browser tab is still descriptive

## 10. Simplify Entry Point

- [x] 10.1 Evaluate whether to delete index.html or rename login.html to index.html
- [x] 10.2 Implement chosen approach (remove redirect or rename file)
- [x] 10.3 Verify login page is accessible at chosen URL

## 11. Manual Testing

- [ ] 11.1 Test login flow end-to-end (form submit → redirect → user-info display)
- [ ] 11.2 Test create user flow end-to-end (form submit → redirect → user-info display)
- [ ] 11.3 Verify theme switches automatically with OS preference change
- [ ] 11.4 Test browser back button after login (no redirect loop)
- [ ] 11.5 Test browser forward button behavior
- [ ] 11.6 Test direct URL access to user-info.html with valid user_id
- [ ] 11.7 Test direct URL access to user-info.html without user_id (should redirect to login)
- [ ] 11.8 Test error cases (invalid credentials, duplicate username)
- [ ] 11.9 Verify no localStorage usage exists (check browser devtools)
- [ ] 11.10 Test with empty form fields (HTML5 validation should prevent submission)

## 12. Verify Success Criteria

- [ ] 12.1 Count JavaScript lines - verify ≤20 lines total (excluding comments/blanks)
- [ ] 12.2 Verify forms submit natively without fetch API
- [ ] 12.3 Verify backend returns 303 redirects for POST endpoints
- [ ] 12.4 Verify theme handled by CSS media queries only
- [ ] 12.5 Verify no localStorage usage exists
- [ ] 12.6 Verify all existing functionality works (login, create user, view info)
- [ ] 12.7 Run all tests: `cargo test` - verify tests pass
