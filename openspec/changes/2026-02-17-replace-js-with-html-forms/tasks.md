# Tasks: Replace JavaScript Form Handling with HTML Forms and Server Redirects

## Phase 1: Backend Modifications

### Task 1.1: Update Login Endpoint to Return Redirects
**File:** `src/rust/main.rs`
**Function:** `login()`

**Changes:**
- [ ] Change success response from `HttpResponse::Ok().body(user_id)` to `HttpResponse::SeeOther()` with redirect to `/user-info.html?user_id={id}`
- [ ] Change error responses from JSON to redirects to `/index.html?error={error_type}`
- [ ] Map all error cases:
  - Invalid credentials → `error=invalid_credentials`
  - Database connection error → `error=database_error`
  - Other errors → `error=server_error`

**Acceptance Criteria:**
- Login endpoint returns 303 status code on both success and failure
- Success redirect includes user_id in query parameter
- Error redirect includes error type in query parameter
- All existing logging remains intact

---

### Task 1.2: Update Create User Endpoint to Return Redirects
**File:** `src/rust/main.rs`
**Function:** `create_user()`

**Changes:**
- [ ] Change success response from `HttpResponse::Ok().body(user_id)` to `HttpResponse::SeeOther()` with redirect to `/user-info.html?user_id={id}`
- [ ] Change error responses from JSON to redirects to `/create-user.html?error={error_type}`
- [ ] Map all error cases:
  - Duplicate username → `error=duplicate_username`
  - Validation errors → `error=validation_error`
  - Database connection error → `error=database_error`
  - Other errors → `error=server_error`

**Acceptance Criteria:**
- Create user endpoint returns 303 status code on both success and failure
- Success redirect includes user_id in query parameter
- Error redirect includes appropriate error type
- All validation logic preserved
- All existing logging remains intact

---

### Task 1.3: Keep User Info Endpoint Unchanged
**File:** `src/rust/main.rs`
**Function:** `get_user_info()`

**Changes:**
- [ ] No changes required - endpoint still returns plain text greeting
- [ ] Verify it continues to work with existing fetch from client

**Acceptance Criteria:**
- Endpoint returns 200 with plain text greeting
- No breaking changes to API contract

---

## Phase 2: Frontend Modifications

### Task 2.1: Update Login Page (index.html)
**File:** `src/web/index.html`

**Changes:**
- [ ] Remove the entire `<script>` tag that handles form submission
- [ ] Keep the error message div structure
- [ ] Add minimal script to display error from query parameter
- [ ] Update error message CSS class handling

**Script to Add:**
```html
<script>
  const urlParams = new URLSearchParams(window.location.search);
  const error = urlParams.get('error');
  const errorElement = document.getElementById('errorMessage');
  
  if (error) {
    const messages = {
      'invalid_credentials': 'Invalid username or password. Please try again.',
      'database_error': 'Database connection failed. Please try again later.',
      'server_error': 'An unexpected error occurred. Please try again.'
    };
    errorElement.textContent = messages[error] || 'An error occurred. Please try again.';
    errorElement.classList.remove('hidden');
  }
</script>
```

**Acceptance Criteria:**
- Form submits directly to `/api/login` without JavaScript interception
- Error messages display when query parameter present
- Page works with JavaScript disabled (form submission still works, just no error display)
- Form retains all existing fields and attributes

---

### Task 2.2: Update Create User Page (create-user.html)
**File:** `src/web/create-user.html`

**Changes:**
- [ ] Remove the entire `<script>` tag that handles form submission
- [ ] Keep the error message div structure
- [ ] Add minimal script to display error from query parameter
- [ ] Update error message CSS class handling

**Script to Add:**
```html
<script>
  const urlParams = new URLSearchParams(window.location.search);
  const error = urlParams.get('error');
  const errorElement = document.getElementById('errorMessage');
  
  if (error) {
    const messages = {
      'duplicate_username': 'Username already exists. Please choose another.',
      'validation_error': 'Please check your input and try again.',
      'database_error': 'Database connection failed. Please try again later.',
      'server_error': 'An unexpected error occurred. Please try again.'
    };
    errorElement.textContent = messages[error] || 'An error occurred. Please try again.';
    errorElement.classList.remove('hidden');
  }
</script>
```

**Acceptance Criteria:**
- Form submits directly to `/api/create-user` without JavaScript interception
- Error messages display when query parameter present
- Page works with JavaScript disabled (form submission still works)
- Form retains all existing fields and attributes

---

### Task 2.3: Keep User Info Page Unchanged (Recommended)
**File:** `src/web/user-info.html`

**Changes:**
- [ ] No changes - keep existing JavaScript for progressive enhancement
- [ ] Verify it continues to work with redirects from login/create-user

**Alternative (Optional - No JavaScript):**
- [ ] Replace JavaScript with simpler server-rendered approach
- [ ] Use iframe or object tag to embed API response
- [ ] This is lower priority and can be done separately

**Acceptance Criteria:**
- Page displays user greeting when accessed with user_id query parameter
- Page redirects to login if no user_id present
- Works with redirects from modified login/create-user endpoints

---

### Task 2.4: Update CSS for Error Messages
**File:** `src/web/style.css`

**Changes:**
- [ ] Ensure `.error-message` class has appropriate styling
- [ ] Verify `.hidden` class properly hides elements
- [ ] Add `:empty` pseudo-class rule to hide empty error divs

**CSS to Verify/Add:**
```css
.error-message {
  background-color: #fee;
  color: #c33;
  padding: 12px;
  border-radius: 4px;
  margin-bottom: 16px;
  border: 1px solid #fcc;
}

.error-message:empty {
  display: none;
}

.hidden {
  display: none;
}
```

**Acceptance Criteria:**
- Error messages display with appropriate styling
- Empty error divs don't show
- Hidden class properly hides elements

---

## Phase 3: Testing

### Task 3.1: Update Backend Tests
**File:** `src/rust/tests/handler_tests.rs` (or similar)

**Changes:**
- [ ] Update `test_login_success` to expect 303 status and Location header
- [ ] Update `test_login_incorrect_password` to expect 303 with error parameter
- [ ] Update `test_create_user_success` to expect 303 status and Location header
- [ ] Update `test_create_user_duplicate_username` to expect 303 with error parameter
- [ ] Add new test for validating redirect Location header format

**Example Test:**
```rust
#[actix_rt::test]
async fn test_login_success_redirects_to_user_info() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::post()
        .uri("/api/login")
        .set_form(&LoginPayload {
            username: "testuser".to_string(),
            password: "password".to_string(),
        })
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let location = resp.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.starts_with("/user-info.html?user_id="));
}
```

**Acceptance Criteria:**
- All existing tests updated and passing
- New redirect tests added
- Test coverage maintained

---

### Task 3.2: Manual Testing
**Test Cases:**

**Happy Path:**
- [ ] Test 1: Open `/index.html`, submit valid credentials, verify redirect to user-info page
- [ ] Test 2: Open `/create-user.html`, submit new user, verify redirect to user-info page
- [ ] Test 3: User info page displays greeting correctly

**Error Cases:**
- [ ] Test 4: Submit invalid credentials, verify redirect back to login with error message
- [ ] Test 5: Submit duplicate username, verify redirect back to create-user with error message
- [ ] Test 6: Error messages display correctly on the page

**JavaScript Disabled:**
- [ ] Test 7: Disable JavaScript in browser, test login flow - form should submit
- [ ] Test 8: Disable JavaScript, test create user flow - form should submit
- [ ] Test 9: With JS disabled, error parameters should be in URL (but not displayed)

**Acceptance Criteria:**
- All test cases pass
- Application works without JavaScript for core functionality
- Error messages display when JavaScript enabled

---

## Phase 4: Documentation

### Task 4.1: Update Architecture Documentation
**File:** `src/web/README.md` or main `README.md`

**Changes:**
- [ ] Document the POST-Redirect-GET pattern used
- [ ] Note that forms now work without JavaScript
- [ ] Explain error handling via query parameters
- [ ] Update any diagrams or flow descriptions

**Acceptance Criteria:**
- Documentation accurately reflects new architecture
- Developers understand the redirect pattern

---

### Task 4.2: Update API Documentation
**File:** API documentation or inline comments

**Changes:**
- [ ] Update `/api/login` endpoint documentation
  - Response: 303 See Other with Location header
  - Success: `/user-info.html?user_id={id}`
  - Error: `/index.html?error={error_type}`
- [ ] Update `/api/create-user` endpoint documentation
  - Response: 303 See Other with Location header
  - Success: `/user-info.html?user_id={id}`
  - Error: `/create-user.html?error={error_type}`

**Acceptance Criteria:**
- API documentation matches implementation
- Examples show redirect responses

---

## Phase 5: Validation and Cleanup

### Task 5.1: Code Review Checklist
- [ ] All JavaScript removed from login form (except error display)
- [ ] All JavaScript removed from create-user form (except error display)
- [ ] Backend returns appropriate HTTP status codes (303)
- [ ] Error messages are user-friendly and informative
- [ ] No security issues (error messages don't leak sensitive info)
- [ ] Logging preserved for all operations
- [ ] Tests updated and passing

### Task 5.2: Final Verification
- [ ] Run full test suite: `cargo test`
- [ ] Start server: `cargo run`
- [ ] Test complete user flow manually
- [ ] Test with JavaScript disabled
- [ ] Verify in multiple browsers (Chrome, Firefox, Safari)
- [ ] Check mobile responsiveness

---

## Rollback Plan

If issues arise during implementation:

1. **Backend rollback**: Revert endpoint changes to return text/JSON responses
2. **Frontend rollback**: Restore JavaScript form handlers
3. **Partial rollback**: Keep one endpoint updated, revert the other

**Rollback indicators:**
- Tests failing significantly
- Major user experience degradation
- Unforeseen security issues

---

## Estimated Effort

| Phase | Task | Estimated Time |
|-------|------|----------------|
| 1.1 | Update login endpoint | 30 minutes |
| 1.2 | Update create-user endpoint | 30 minutes |
| 1.3 | Verify user-info endpoint | 5 minutes |
| 2.1 | Update index.html | 20 minutes |
| 2.2 | Update create-user.html | 20 minutes |
| 2.3 | Verify user-info.html | 5 minutes |
| 2.4 | Update CSS | 10 minutes |
| 3.1 | Update backend tests | 45 minutes |
| 3.2 | Manual testing | 30 minutes |
| 4.1 | Update documentation | 20 minutes |
| 4.2 | Update API docs | 15 minutes |
| 5.1 | Code review | 15 minutes |
| 5.2 | Final verification | 30 minutes |
| **Total** | | **~4-5 hours** |

---

## Success Metrics

- ✅ All tests passing
- ✅ Application works without JavaScript (for core forms)
- ✅ No increase in error rates
- ✅ Reduced JavaScript bundle size
- ✅ Improved accessibility score
- ✅ Simpler codebase (less client-side state management)
