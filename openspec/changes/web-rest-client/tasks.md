## 1. Project Setup and Configuration

- [x] 1.1 Initialize React+TypeScript project (Create React App or Vite)
- [x] 1.2 Configure build tooling and development environment
- [x] 1.3 Install dependencies (React Router, axios, other utilities)
- [x] 1.4 Set up project folder structure for components, pages, utilities
- [x] 1.5 Configure TypeScript paths and type definitions

## 2. Core UI Components and Utilities

- [x] 2.1 Create Form component wrapper with validation support
- [x] 2.2 Create re-usable Input field component with labels and error display
- [x] 2.3 Create Button component matching design system
- [x] 2.4 Create Error message/toast notification component
- [x] 2.5 Create API client utility with axios (error handling, base URL)
- [x] 2.6 Create local storage utility for session management (user_id)

## 3. Router and Navigation Setup

- [x] 3.1 Configure React Router with routes for three pages
- [x] 3.2 Create navigation helper functions for redirects
- [x] 3.3 Set up page layout component (header, footer if needed)
- [x] 3.4 Implement redirect logic to login if user_id is missing

## 4. UI-Create-User Page Implementation

- [x] 4.1 Create CreateUserPage component structure
- [x] 4.2 Implement form state management for all 7 user fields
- [x] 4.3 Add front-end validation for required fields and email format
- [x] 4.4 Implement POST /api/users API call with error handling
- [x] 4.5 Store returned user_id in browser storage
- [x] 4.6 Implement redirect to UI-user-info page on success
- [x] 4.7 Display user-friendly error messages on API failure

## 5. UI-Login Page Implementation

- [x] 5.1 Create LoginPage component structure
- [x] 5.2 Implement form state management for username and password
- [x] 5.3 Add front-end validation for required fields
- [x] 5.4 Implement POST /api/login API call with error handling
- [x] 5.5 Store returned user_id in browser storage
- [x] 5.6 Implement redirect to UI-user-info page on success
- [x] 5.7 Display authentication error messages ("Invalid username or password")

## 6. UI-User-Info Page Implementation

- [x] 6.1 Create UserInfoPage component structure
- [x] 6.2 Implement page initialization to read user_id from storage
- [x] 6.3 Implement GET /api/users/{user_id} API call on page load
- [x] 6.4 Create data display component for user information
- [x] 6.5 Format and present all user fields (username, first/last name, email, title, hobby)
- [x] 6.6 Handle "user not found" error (404 response)
- [x] 6.7 Handle server errors with retry option
- [x] 6.8 Display redirect link to login if user_id is missing

## 7. CORS and API Integration

- [x] 7.1 Verify Rust service has CORS headers configured
- [x] 7.2 Test API endpoints from browser (create-user, login, get-user-info)
- [x] 7.3 Configure axios for proper content-type and request format
- [x] 7.4 Handle API response parsing and error detection

## 8. Session Persistence and Edge Cases

- [x] 8.1 Implement localStorage for user_id persistence across refreshes
- [x] 8.2 Add session timeout handling if needed
- [x] 8.3 Handle missing user_id gracefully (redirect to login)
- [x] 8.4 Clear session on logout/redirect scenarios

## 9. Testing and Validation

- [ ] 9.1 Write unit tests for form validation functions
- [ ] 9.2 Write integration tests for API calls (mock or real endpoints)
- [ ] 9.3 Test happy path: create user → login → view profile
- [ ] 9.4 Test error scenarios (invalid input, network errors, server errors)
- [ ] 9.5 Test browser storage and session persistence
- [ ] 9.6 Manual testing across different browsers

## 10. Deployment and Documentation

- [x] 10.1 Build production bundle
- [x] 10.2 Configure deployment target (same server as Rust service or separate)
- [x] 10.3 Set production API endpoint URLs
- [ ] 10.4 Deploy web application
- [ ] 10.5 Verify CORS configuration in production
- [x] 10.6 Create deployment documentation
