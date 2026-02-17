# Design: Replace JavaScript Form Handling with HTML Forms and Server Redirects

## Architecture Overview

```
┌─────────────┐     POST      ┌──────────────┐    Success    ┌──────────────┐
│ Login Form  │ ───────────> │   /api/login │ ────────────> │  user-info   │
│ index.html  │              │   endpoint   │   (redirect)  │   .html      │
└─────────────┘              └──────────────┘               └──────────────┘
                                    │
                                    │ Error
                                    ▼
                             ┌──────────────┐
                             │ index.html?  │
                             │ error=...    │
                             └──────────────┘
```

## Component Design

### 1. Backend API Endpoints

#### Login Endpoint Modification

**Before:**
```rust
async fn login(
    state: web::Data<AppState>,
    payload: web::Form<LoginPayload>,
) -> impl Responder {
    match state.db.authenticate_user(&payload.username).await {
        Ok((user_id, stored_password)) => {
            if stored_password == payload.password {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(user_id.to_string())
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "INVALID_CREDENTIALS".to_string(),
                    message: "Invalid username or password".to_string(),
                })
            }
        }
        // ... error cases
    }
}
```

**After:**
```rust
async fn login(
    state: web::Data<AppState>,
    payload: web::Form<LoginPayload>,
) -> impl Responder {
    match state.db.authenticate_user(&payload.username).await {
        Ok((user_id, stored_password)) => {
            if stored_password == payload.password {
                log_info!(state.http_client, "login_user", payload.username, "Successful login");
                // Redirect to user info page with user_id
                HttpResponse::SeeOther()
                    .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                    .finish()
            } else {
                log_info!(state.http_client, "login_user", payload.username, "Invalid password");
                // Redirect back to login with error
                HttpResponse::SeeOther()
                    .append_header(("Location", "/index.html?error=invalid_credentials"))
                    .finish()
            }
        }
        Err(DatabaseError::UserNotFound) => {
            log_info!(state.http_client, "login_user", payload.username, "User not found");
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=invalid_credentials"))
                .finish()
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "login_user", payload.username, "Database connection error");
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=database_error"))
                .finish()
        }
        Err(e) => {
            log_error!(state.http_client, "login_user", payload.username, "Error: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=server_error"))
                .finish()
        }
    }
}
```

#### Create User Endpoint Modification

**After:**
```rust
async fn create_user(
    state: web::Data<AppState>,
    payload: web::Form<CreateUserPayload>,
) -> impl Responder {
    // ... validation code ...
    
    match state.db.create_user(&create_request).await {
        Ok(user_id) => {
            log_info!(state.http_client, "create_user", payload.username, "User created with ID: {}", user_id);
            HttpResponse::SeeOther()
                .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                .finish()
        }
        Err(DatabaseError::DuplicateUsername) => {
            log_info!(state.http_client, "create_user", payload.username, "Duplicate username");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=duplicate_username"))
                .finish()
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "create_user", payload.username, "Database error");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=database_error"))
                .finish()
        }
        Err(e) => {
            log_error!(state.http_client, "create_user", payload.username, "Error: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=server_error"))
                .finish()
        }
    }
}
```

### 2. Frontend HTML Changes

#### index.html (Login Page)

**Changes:**
1. Remove `<script>` tag entirely
2. Add error message display based on query parameter
3. Form already has correct attributes (method="POST", action="/api/login")

**Structure:**
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Login - User Management</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <div class="page-container">
    <div class="card">
      <h1>Welcome Back</h1>
      <h2>Login to your account</h2>

      <!-- Error message display - always visible but only shows content if error param present -->
      <div id="errorMessage" class="error-message"></div>

      <form method="POST" action="/api/login">
        <div class="form-group">
          <label for="username">Username <span class="required">*</span></label>
          <input type="text" id="username" name="username" required placeholder="Enter username">
        </div>

        <div class="form-group">
          <label for="password">Password <span class="required">*</span></label>
          <input type="password" id="password" name="password" required placeholder="Enter password">
        </div>

        <button type="submit">Login</button>
      </form>

      <div class="link-container">
        <p>Don't have an account? <a href="create-user.html">Create New User</a></p>
      </div>
    </div>
  </div>

  <!-- Minimal script to display error from query parameter -->
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
</body>
</html>
```

**Note:** This includes minimal JavaScript only for displaying error messages. The core form submission works without JavaScript.

#### create-user.html (Create User Page)

Similar changes to login page:
1. Remove form submission JavaScript
2. Add error message display from query parameter
3. Keep form attributes as-is

#### user-info.html (User Info Page)

**Option 1: Keep Minimal JavaScript (Recommended)**
- Keep the existing fetch logic
- This is progressive enhancement
- Core login/create flows work without JS

**Option 2: No JavaScript Alternative**
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>User Info - User Management</title>
  <link rel="stylesheet" href="style.css">
</head>
<body>
  <div class="page-container">
    <div class="card">
      <!-- Use object tag to embed API response -->
      <object data="" id="userGreeting" type="text/plain">
        <p>Loading user information...</p>
      </object>
    </div>
  </div>

  <script>
    // Minimal script to set data URL from query param
    const urlParams = new URLSearchParams(window.location.search);
    const userId = urlParams.get('user_id');
    if (userId) {
      document.getElementById('userGreeting').data = `/api/users/${userId}`;
    } else {
      window.location.href = '/index.html';
    }
  </script>
</body>
</html>
```

### 3. Error Handling

#### Error Types and Messages

| Error Code | User Message | Redirect Target |
|------------|-------------|-----------------|
| `invalid_credentials` | "Invalid username or password. Please try again." | `/index.html?error=invalid_credentials` |
| `duplicate_username` | "Username already exists. Please choose another." | `/create-user.html?error=duplicate_username` |
| `database_error` | "Database connection failed. Please try again later." | `[form]?error=database_error` |
| `validation_error` | "Please check your input and try again." | `[form]?error=validation_error` |
| `server_error` | "An unexpected error occurred. Please try again." | `[form]?error=server_error` |

#### CSS for Error Display

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
```

### 4. HTTP Status Codes

| Scenario | Status Code | Response |
|----------|-------------|----------|
| Successful login | 303 See Other | Redirect to `/user-info.html?user_id={id}` |
| Invalid credentials | 303 See Other | Redirect to `/index.html?error=invalid_credentials` |
| Successful user creation | 303 See Other | Redirect to `/user-info.html?user_id={id}` |
| Duplicate username | 303 See Other | Redirect to `/create-user.html?error=duplicate_username` |
| Server error | 303 See Other | Redirect to `[form]?error=server_error` |

**Why 303 See Other?**
- Prevents form resubmission on browser refresh (POST-Redirect-GET pattern)
- Browser changes method to GET for the redirect
- Standard practice for form submissions

### 5. Backward Compatibility

**API Endpoints:**
- `/api/login` - Changes from returning text to returning redirects
- `/api/create-user` - Changes from returning text to returning redirects
- `/api/users/{id}` - **No changes** (still returns text greeting)

**Impact on Tests:**
- Update endpoint tests to expect 303 redirects
- Verify redirect Location headers
- Existing API response format tests may need adjustment

### 6. Testing Strategy

**Backend Tests:**
```rust
#[test]
async fn test_login_success_redirects() {
    let response = test_client.post("/api/login")
        .form(&login_data)
        .send()
        .await;
    
    assert_eq!(response.status(), 303);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        "/user-info.html?user_id=1"
    );
}

#[test]
async fn test_login_failure_redirects() {
    let response = test_client.post("/api/login")
        .form(&invalid_login)
        .send()
        .await;
    
    assert_eq!(response.status(), 303);
    assert!(response.headers().get("Location").unwrap()
        .contains("error=invalid_credentials"));
}
```

**Manual Tests:**
1. Submit login form → Should redirect to user-info page
2. Submit with wrong password → Should redirect back with error
3. Create new user → Should redirect to user-info page
4. Create duplicate username → Should redirect back with error
5. Disable JavaScript → All forms should still work

## Security Considerations

1. **CSRF Protection**: Not implemented (out of scope for this change)
   - Future enhancement: Add CSRF tokens to forms
   
2. **Error Information Disclosure**: Error messages are generic
   - "Invalid credentials" instead of "User not found" vs "Wrong password"
   
3. **Query Parameter Validation**: Error parameter values are whitelisted
   - Only display predefined error messages

4. **XSS Prevention**: Error messages are not dynamically rendered from query params
   - Use predefined message mapping

## Performance Considerations

1. **Reduced JavaScript**: Faster page load (no JS parsing/execution for forms)
2. **Full Page Reloads**: Slightly slower than AJAX, but offset by faster initial load
3. **Browser Caching**: Static HTML pages cache better than dynamic JS responses
4. **User-Info Page**: Keep fetch for progressive enhancement (minimal JS)

## Accessibility Improvements

1. **Screen Readers**: Standard form submissions announced correctly
2. **Keyboard Navigation**: No custom JavaScript event handling needed
3. **No JavaScript**: Core functionality works without JS
4. **Focus Management**: Browser handles focus after redirect

## Migration Path

**Phase 1: Backend Changes**
1. Modify login endpoint to return redirects
2. Modify create-user endpoint to return redirects
3. Update tests

**Phase 2: Frontend Changes**
1. Update index.html (remove JS, add error display)
2. Update create-user.html (remove JS, add error display)
3. Update CSS for error messages

**Phase 3: User Info Page (Optional)**
- Keep existing implementation (progressive enhancement)
- OR implement no-JS alternative if needed

**Phase 4: Validation**
1. Test all user flows manually
2. Test with JavaScript disabled
3. Verify all tests pass

## References

- [POST-Redirect-GET Pattern](https://en.wikipedia.org/wiki/Post/Redirect/Get)
- [HTTP 303 See Other](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/303)
- [Progressive Enhancement](https://developer.mozilla.org/en-US/docs/Glossary/Progressive_Enhancement)
