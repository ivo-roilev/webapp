## 1. Backend: Create User Info Formatter Module

- [ ] 1.1 Create new file `src/rust/user_info_formatter.rs` for text formatting logic
- [ ] 1.2 Import necessary types (User, database types) into the new module
- [ ] 1.3 Add module declaration in `src/rust/main.rs` (`mod user_info_formatter;`)

## 2. Backend: Implement Name Construction Logic

- [ ] 2.1 Create helper function `construct_name(first_name: Option<String>, last_name: Option<String>, username: String) -> String`
- [ ] 2.2 Implement prioritized fallback: full name → first → last → username
- [ ] 2.3 Add unit tests for name construction edge cases (all combinations)

## 3. Backend: Implement Text Formatting Function

- [ ] 3.1 Create `format_user_greeting(user: User) -> String` function
- [ ] 3.2 Implement greeting section: "Hello [Title] [Name], welcome!"
- [ ] 3.3 Implement conditional title prefix in greeting (include only if present)
- [ ] 3.4 Implement hobby notification clause: "If we hear interesting news about [Hobby]"
- [ ] 3.5 Implement email notification extension: "we will let you know at [Email]!"
- [ ] 3.6 Handle all conditional combinations (hobby+email, hobby only, neither)
- [ ] 3.7 Add unit tests for all field combinations per spec examples

## 4. Backend: Update Get User Info Handler

- [ ] 4.1 Modify `get_user_info` handler in `src/rust/main.rs` to call `format_user_greeting()`
- [ ] 4.2 Remove `UserInfoResponse` struct serialization logic
- [ ] 4.3 Change response to `HttpResponse::Ok().content_type("text/plain; charset=utf-8").body(formatted_text)`
- [ ] 4.4 Update 404 error to return plain text: "User with ID {user_id} not found"
- [ ] 4.5 Update 400 invalid format error to return plain text: "user_id must be a valid integer"
- [ ] 4.6 Update 400 negative ID error to return plain text: "user_id must be a positive integer"
- [ ] 4.7 Update 503 database error to return plain text: "Database connection failed"
- [ ] 4.8 Update 500 server error to return plain text: "Failed to fetch user"
- [ ] 4.9 Ensure all error responses use Content-Type: text/plain

## 5. Frontend: Update User Info Page Component

- [ ] 5.1 Locate user info page component in `src/web/src/pages/` or `src/web/src/components/`
- [ ] 5.2 Remove JSON parsing logic from API response handling
- [ ] 5.3 Update to render response text directly (response.text() instead of response.json())
- [ ] 5.4 Ensure text is displayed in centered, readable format
- [ ] 5.5 Verify Content-Type expectation is text/plain (update fetch headers if needed)

## 6. Frontend: Update Error Handling

- [ ] 6.1 Update error display to show plain text error messages from API
- [ ] 6.2 Remove any JSON error parsing (error.message, error.error fields)
- [ ] 6.3 Display error text directly from response body
- [ ] 6.4 Ensure error navigation options (return to login/create user) still work

## 7. Testing and Verification

- [ ] 7.1 Test full profile (all fields): "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
- [ ] 7.2 Test minimal profile (username only): "Hello jdoe, welcome!"
- [ ] 7.3 Test no hobby scenario: "Hello Software Engineer John Doe, welcome!"
- [ ] 7.4 Test no title scenario: "Hello John Doe, welcome! If we hear interesting news about hiking, we will let you know at john@email.com!"
- [ ] 7.5 Test no email scenario: "Hello Software Engineer John Doe, welcome! If we hear interesting news about hiking, we will let you know!"
- [ ] 7.6 Test 404 error response displays correctly
- [ ] 7.7 Test 400 invalid format error displays correctly
- [ ] 7.8 Test 400 negative ID error displays correctly
- [ ] 7.9 Verify frontend renders all greeting formats without breaking
- [ ] 7.10 Verify error messages display properly in UI with navigation options
