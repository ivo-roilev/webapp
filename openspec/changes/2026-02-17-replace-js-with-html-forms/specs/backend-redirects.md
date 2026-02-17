# Spec: Backend Redirect Endpoints

## Overview
Modify the `/api/login` and `/api/create-user` endpoints to return HTTP 303 redirects instead of plain text or JSON responses, following the POST-Redirect-GET pattern.

## Endpoints

### 1. POST /api/login

**Current Behavior:**
- Success: Returns `200 OK` with plain text body containing user ID
- Error: Returns `401 Unauthorized` or other status with JSON error object

**New Behavior:**
- Success: Returns `303 See Other` with `Location: /user-info.html?user_id={id}`
- Invalid credentials: Returns `303 See Other` with `Location: /index.html?error=invalid_credentials`
- Database error: Returns `303 See Other` with `Location: /index.html?error=database_error`
- Server error: Returns `303 See Other` with `Location: /index.html?error=server_error`

**Implementation:**

```rust
async fn login(
    state: web::Data<AppState>,
    payload: web::Form<LoginPayload>,
) -> impl Responder {
    log_info!(state.http_client, "login_user", payload.username, "Login attempt");

    // Validate required fields
    if payload.username.is_empty() {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/index.html?error=validation_error"))
            .finish();
    }

    if payload.password.is_empty() {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/index.html?error=validation_error"))
            .finish();
    }

    match state.db.authenticate_user(&payload.username).await {
        Ok((user_id, stored_password)) => {
            if stored_password == payload.password {
                log_info!(state.http_client, "login_user", payload.username, "Successful login");
                HttpResponse::SeeOther()
                    .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                    .finish()
            } else {
                log_info!(state.http_client, "login_user", payload.username, "Invalid password");
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
            log_error!(state.http_client, "login_user", payload.username, "Error during login: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/index.html?error=server_error"))
                .finish()
        }
    }
}
```

### 2. POST /api/create-user

**Current Behavior:**
- Success: Returns `200 OK` with plain text body containing user ID
- Error: Returns `400 Bad Request`, `409 Conflict`, or `500 Internal Server Error` with JSON

**New Behavior:**
- Success: Returns `303 See Other` with `Location: /user-info.html?user_id={id}`
- Validation error: Returns `303 See Other` with `Location: /create-user.html?error=validation_error`
- Duplicate username: Returns `303 See Other` with `Location: /create-user.html?error=duplicate_username`
- Database error: Returns `303 See Other` with `Location: /create-user.html?error=database_error`
- Server error: Returns `303 See Other` with `Location: /create-user.html?error=server_error`

**Implementation:**

```rust
async fn create_user(
    state: web::Data<AppState>,
    payload: web::Form<CreateUserPayload>,
) -> impl Responder {
    log_info!(state.http_client, "create_user", payload.username, "Creating new user");

    // Validate required fields
    if payload.username.is_empty() || payload.username.len() > 16 {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/create-user.html?error=validation_error"))
            .finish();
    }

    if payload.password.is_empty() || payload.password.len() > 255 {
        return HttpResponse::SeeOther()
            .append_header(("Location", "/create-user.html?error=validation_error"))
            .finish();
    }

    // Validate optional fields length
    if let Some(ref first_name) = payload.first_name {
        if first_name.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref last_name) = payload.last_name {
        if last_name.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref email) = payload.email {
        if email.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref title) = payload.title {
        if title.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    if let Some(ref hobby) = payload.hobby {
        if hobby.len() > 255 {
            return HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=validation_error"))
                .finish();
        }
    }

    let mut metadata = Vec::new();
    if let Some(ref title) = payload.title {
        metadata.push(UserMetadata {
            parent_property: None,
            property: "title".to_string(),
            value: Some(title.clone()),
        });
    }
    if let Some(ref hobby) = payload.hobby {
        metadata.push(UserMetadata {
            parent_property: None,
            property: "hobby".to_string(),
            value: Some(hobby.clone()),
        });
    }
    if let Some(ref extra) = payload.extra_metadata {
        metadata.extend(extra.clone());
    }

    let create_request = CreateUserRequest {
        username: payload.username.clone(),
        password: payload.password.clone(),
        profile: Some(UserProfile {
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            email: payload.email.clone(),
        }),
        metadata,
    };

    match state.db.create_user(&create_request).await {
        Ok(user_id) => {
            log_info!(state.http_client, "create_user", payload.username, "User created successfully with ID: {}", user_id);
            HttpResponse::SeeOther()
                .append_header(("Location", format!("/user-info.html?user_id={}", user_id)))
                .finish()
        }
        Err(DatabaseError::DuplicateUsername) => {
            log_info!(state.http_client, "create_user", payload.username, "Username already exists");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=duplicate_username"))
                .finish()
        }
        Err(DatabaseError::ConnectionError(_)) => {
            log_error!(state.http_client, "create_user", payload.username, "Database connection error");
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=database_error"))
                .finish()
        }
        Err(e) => {
            log_error!(state.http_client, "create_user", payload.username, "Error creating user: {:?}", e);
            HttpResponse::SeeOther()
                .append_header(("Location", "/create-user.html?error=server_error"))
                .finish()
        }
    }
}
```

## Error Code Mapping

| Original Error | HTTP Status | Redirect Location |
|----------------|-------------|-------------------|
| Invalid credentials (login) | 401 → 303 | `/index.html?error=invalid_credentials` |
| User not found (login) | 401 → 303 | `/index.html?error=invalid_credentials` |
| Duplicate username (create) | 409 → 303 | `/create-user.html?error=duplicate_username` |
| Validation error | 400 → 303 | `[form]?error=validation_error` |
| Database connection error | 503 → 303 | `[form]?error=database_error` |
| Other server errors | 500 → 303 | `[form]?error=server_error` |

## Testing

### Unit Tests

```rust
#[actix_rt::test]
async fn test_login_success_returns_redirect() {
    let state = setup_test_state().await;
    let payload = web::Form(LoginPayload {
        username: "testuser".to_string(),
        password: "password".to_string(),
    });

    let response = login(state, payload).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response.headers().get("Location").unwrap().to_str().unwrap();
    assert!(location.starts_with("/user-info.html?user_id="));
}

#[actix_rt::test]
async fn test_login_failure_returns_redirect_with_error() {
    let state = setup_test_state().await;
    let payload = web::Form(LoginPayload {
        username: "testuser".to_string(),
        password: "wrongpassword".to_string(),
    });

    let response = login(state, payload).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response.headers().get("Location").unwrap().to_str().unwrap();
    assert_eq!(location, "/index.html?error=invalid_credentials");
}

#[actix_rt::test]
async fn test_create_user_duplicate_returns_redirect() {
    let state = setup_test_state().await;
    // Create user first
    create_test_user(&state, "duplicate").await;
    
    let payload = web::Form(CreateUserPayload {
        username: "duplicate".to_string(),
        password: "password".to_string(),
        // ... other fields
    });

    let response = create_user(state, payload).await;
    
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    let location = response.headers().get("Location").unwrap().to_str().unwrap();
    assert_eq!(location, "/create-user.html?error=duplicate_username");
}
```

### Integration Tests

```rust
#[actix_rt::test]
async fn test_login_redirect_flow() {
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
    assert!(resp.headers().get("Location").is_some());
}
```

## Validation Criteria

- ✅ All success cases return 303 status code
- ✅ All error cases return 303 status code
- ✅ Location header is present in all responses
- ✅ Location header contains correct path and query parameters
- ✅ All logging statements preserved
- ✅ No breaking changes to database interactions
- ✅ User ID correctly included in success redirects
- ✅ Error type correctly included in error redirects

## Security Considerations

- Error messages are generic (don't distinguish between "user not found" and "wrong password")
- User ID is passed in URL query parameter (acceptable for this use case)
- No sensitive information in error parameters
- All validation logic preserved

## Performance Impact

- Minimal performance impact
- One additional HTTP request (redirect) per form submission
- Browser handles redirect automatically
- Follows standard web patterns

## Backward Compatibility

**Breaking Changes:**
- API response format changes from text/JSON to HTTP redirects
- Clients expecting JSON responses will need updates
- This is acceptable for HTML form consumers

**Non-Breaking:**
- `/api/users/{id}` endpoint unchanged
- Database queries unchanged
- Validation logic unchanged
