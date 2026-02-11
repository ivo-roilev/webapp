## 1. Project Setup and Configuration

- [ ] 1.1 Initialize React+TypeScript project (Create React App or Vite)
- [ ] 1.2 Configure build tooling and development environment
- [ ] 1.3 Install dependencies (React Router, axios, other utilities)
- [ ] 1.4 Set up project folder structure for components, pages, utilities
- [ ] 1.5 Configure TypeScript paths and type definitions

## 2. Core UI Components and Utilities

- [ ] 2.1 Create Form component wrapper with validation support
- [ ] 2.2 Create re-usable Input field component with labels and error display
- [ ] 2.3 Create Button component matching design system
- [ ] 2.4 Create Error message/toast notification component
- [ ] 2.5 Create API client utility with axios (error handling, base URL)
- [ ] 2.6 Create local storage utility for session management (user_id)

## 3. Router and Navigation Setup

- [ ] 3.1 Configure React Router with routes for three pages
- [ ] 3.2 Create navigation helper functions for redirects
- [ ] 3.3 Set up page layout component (header, footer if needed)
- [ ] 3.4 Implement redirect logic to login if user_id is missing

## 4. UI-Create-User Page Implementation

- [ ] 4.1 Create CreateUserPage component structure
- [ ] 4.2 Implement form state management for all 7 user fields
- [ ] 4.3 Add front-end validation for required fields and email format
- [ ] 4.4 Implement POST /api/users API call with error handling
- [ ] 4.5 Store returned user_id in browser storage
- [ ] 4.6 Implement redirect to UI-user-info page on success
- [ ] 4.7 Display user-friendly error messages on API failure

## 5. UI-Login Page Implementation

- [ ] 5.1 Create LoginPage component structure
- [ ] 5.2 Implement form state management for username and password
- [ ] 5.3 Add front-end validation for required fields
- [ ] 5.4 Implement POST /api/login API call with error handling
- [ ] 5.5 Store returned user_id in browser storage
- [ ] 5.6 Implement redirect to UI-user-info page on success
- [ ] 5.7 Display authentication error messages ("Invalid username or password")

## 6. UI-User-Info Page Implementation

- [ ] 6.1 Create UserInfoPage component structure
- [ ] 6.2 Implement page initialization to read user_id from storage
- [ ] 6.3 Implement GET /api/users/{user_id} API call on page load
- [ ] 6.4 Create data display component for user information
- [ ] 6.5 Format and present all user fields (username, first/last name, email, title, hobby)
- [ ] 6.6 Handle "user not found" error (404 response)
- [ ] 6.7 Handle server errors with retry option
- [ ] 6.8 Display redirect link to login if user_id is missing

## 7. CORS and API Integration

- [ ] 7.1 Verify Rust service has CORS headers configured
- [ ] 7.2 Test API endpoints from browser (create-user, login, get-user-info)
- [ ] 7.3 Configure axios for proper content-type and request format
- [ ] 7.4 Handle API response parsing and error detection

## 8. Session Persistence and Edge Cases

- [ ] 8.1 Implement localStorage for user_id persistence across refreshes
- [ ] 8.2 Add session timeout handling if needed
- [ ] 8.3 Handle missing user_id gracefully (redirect to login)
- [ ] 8.4 Clear session on logout/redirect scenarios

## 9. Testing and Validation

- [ ] 9.1 Write unit tests for form validation functions
- [ ] 9.2 Write integration tests for API calls (mock or real endpoints)
- [ ] 9.3 Test happy path: create user → login → view profile
- [ ] 9.4 Test error scenarios (invalid input, network errors, server errors)
- [ ] 9.5 Test browser storage and session persistence
- [ ] 9.6 Manual testing across different browsers

## 10. Deployment and Documentation

- [ ] 10.1 Build production bundle
- [ ] 10.2 Configure deployment target (same server as Rust service or separate)
- [ ] 10.3 Set production API endpoint URLs
- [ ] 10.4 Deploy web application
- [ ] 10.5 Verify CORS configuration in production
- [ ] 10.6 Create deployment documentation
