## 1. Backend: Create User Info Formatter Module

- [x] 1.1 Create new file `src/rust/user_info_formatter.rs` for text formatting logic
- [x] 1.2 Import necessary types (User, database types) into the new module
- [x] 1.3 Add module declaration in `src/rust/main.rs` (`mod user_info_formatter;`)

## 2. Backend: Implement Name Construction Logic

- [x] 2.1 Create helper function `construct_name(first_name: Option<String>, last_name: Option<String>, username: String) -> String`
- [x] 2.2 Implement prioritized fallback: full name → first → last → username
- [x] 2.3 Add unit tests for name construction edge cases (all combinations)

## 3. Backend: Implement Text Formatting Function

- [x] 3.1 Create `format_user_greeting(user: User) -> String` function
- [x] 3.2 Implement greeting section: "Hello [Title] [Name], welcome!"
- [x] 3.3 Implement conditional title prefix in greeting (include only if present)
- [x] 3.4 Implement hobby notification clause: "If we hear interesting news about [Hobby]"
- [x] 3.5 Implement email notification extension: "we will let you know at [Email]!"
- [x] 3.6 Handle all conditional combinations (hobby+email, hobby only, neither)
- [x] 3.7 Add unit tests for all field combinations per spec examples

## 4. Backend: Update Get User Info Handler

- [x] 4.1 Modify `get_user_info` handler in `src/rust/main.rs` to call `format_user_greeting()`
- [x] 4.2 Remove `UserInfoResponse` struct serialization logic
- [x] 4.3 Change response to `HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(formatted_text)`
- [x] 4.4 Update 404 error to return plain text: "User with ID {user_id} not found"
- [x] 4.5 Update 400 invalid format error to return plain text: "user_id must be a valid integer"
- [x] 4.6 Update 400 negative ID error to return plain text: "user_id must be a positive integer"
- [x] 4.7 Update 503 database error to return plain text: "Database connection failed"
- [x] 4.8 Update 500 server error to return plain text: "Failed to fetch user"
- [x] 4.9 Ensure all error responses use Content-Type: text/plain

## 5. Frontend: Update User Info Page Component

- [x] 5.1 Locate user info page component in `src/web/src/pages/` or `src/web/src/components/`
- [x] 5.2 Remove JSON parsing logic from API response handling
- [x] 5.3 Update to render response text directly (response.text() instead of response.json())
- [x] 5.4 Ensure text is displayed in centered, readable format
- [x] 5.5 Verify Content-Type expectation is text/plain (update fetch headers if needed)

## 6. Frontend: Update Error Handling

- [x] 6.1 Update error display to show plain text error messages from API
- [x] 6.2 Remove any JSON error parsing (error.message, error.error fields)
- [x] 6.3 Display error text directly from response body
- [x] 6.4 Ensure error navigation options (return to login/create user) still work

## 7. Testing and Verification

- [x] 7.1 Test full profile (all fields): "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
- [x] 7.2 Test minimal profile (username only): "Hello jdoe, welcome!"
- [x] 7.3 Test no hobby scenario: "Hello Software Engineer John Doe, welcome!"
- [x] 7.4 Test no title scenario: "Hello John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
- [x] 7.5 Test no email scenario: "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know!"
- [x] 7.6 Test 404 error response displays correctly
- [x] 7.7 Test 400 invalid format error displays correctly
- [x] 7.8 Test 400 negative ID error displays correctly
- [x] 7.9 Verify frontend renders all greeting formats without breaking
- [x] 7.10 Verify error messages display properly in UI with navigation options
