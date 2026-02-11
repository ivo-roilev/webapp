## Context

The Rust-based REST service provides three endpoints for user management:
- `POST /api/users` - Create a new user, returns `user_id`
- `POST /api/login` - Authenticate a user, returns `user_id`
- `GET /api/users/{user_id}` - Retrieve user information

These endpoints are deployed and accessible. This design creates a web interface to allow non-technical users to interact with these APIs through a browser-based UI.

## Goals / Non-Goals

**Goals:**
- Create three interconnected UI pages (create-user, login, user-info) that work seamlessly together
- Provide a simple, intuitive interface for user registration and authentication
- Enable session flow: create user → view profile ; and login → view profile
- Support all user fields: username, password, first_name, last_name, email, title, hobby. While username and password are mandatory, the rest of the fields can be empty.

**Non-Goals:**
- User profile editing or deletion
- Password recovery or reset functionality
- Persistent session management (client-side token handling is out of scope)
- Admin dashboard or user management interface
- Email verification
- Secure connection
- password encryption or encoding.x

## Decisions

**Decision 1: Frontend Framework and Technology Stack**
- **Choice**: React with TypeScript (SPA - Single Page Application)
- **Rationale**: TypeScript provides type safety and IDE support; React enables component reusability across the three pages; SPA avoids full page reloads and provides better UX
- **Alternative Considered**: Vue.js (lighter weight, but React is more widely adopted for team continuity)

**Decision 2: State Management**
- **Choice**: React hooks (useState) with local browser storage for user_id
- **Rationale**: Simple enough for three pages; no external dependency like Redux needed
- **Alternative Considered**: Redux (overkill for this scope; adds complexity)

**Decision 3: HTTP Client Library**
- **Choice**: Native Fetch API or axios
- **Rationale**: Fetch is web standard; axios provides better error handling and interceptor support; choose axios for consistency with team practices
- **Alternative Considered**: Native XMLHttpRequest (verbose and outdated)

**Decision 4: Component Structure**
- **Choice**: Three main page components (CreateUserPage, LoginPage, UserInfoPage) plus shared UI components (Form, Button, Input field wrapper)
- **Rationale**: Clear separation of concerns; page-level components manage routing/state transitions; UI components are reusable
- **Alternative Considered**: Single monolithic component (harder to maintain and test)

**Decision 5: Routing and Navigation**
- **Choice**: React Router for client-side routing
- **Rationale**: Clean URL structure; enables browser back/forward; standard React practice
- **Alternative Considered**: Manual state-based navigation (less user-friendly, harder to bookmark pages)

**Decision 6: CORS and API Integration**
- **Choice**: Assume Rust service is configured to accept requests from the web app domain; use credentials: 'include' if session cookies are required
- **Rationale**: REST service should be accessible from browser; same-origin or CORS headers must be configured on backend
- **Alternative Considered**: Backend proxy (adds complexity; direct calls are simpler)

## Risks / Trade-offs

**Risk 1: CORS Configuration**
- **Description**: If the Rust service doesn't have CORS headers configured, requests from the browser will be blocked
- **Mitigation**: Verify Rust service includes `Access-Control-Allow-Origin` headers; test in development early

**Risk 2: User Experience with Errors**
- **Description**: Network failures or validation errors need clear user feedback
- **Mitigation**: Implement error boundaries and toast notifications for all API failures; validate form inputs before sending

**Risk 3: Session Loss**
- **Description**: Refreshing the page will lose the user_id if stored only in memory
- **Mitigation**: Store user_id in browser localStorage or sessionStorage; retrieve on page load

**Trade-off 1: TypeScript vs Development Speed**
- **Description**: TypeScript adds type safety but requires build tooling
- **Mitigation**: Use Create React App or Vite to handle build complexity automatically

**Trade-off 2: Frontend Validation vs Backend Validation**
- **Description**: Frontend validation improves UX but backend must also validate for security
- **Mitigation**: Implement both; frontend for UX, backend is source of truth

## Migration Plan

1. **Setup**: Initialize React/TypeScript project (Create React App or Vite)
2. **Phase 1**: Build core components and pages (no API integration initially)
3. **Phase 2**: Integrate with REST endpoints; test CORS and error handling
4. **Phase 3**: Add routing and navigation between pages
5. **Phase 4**: Handle session persistence and edge cases
6. **Deployment**: Deploy to web server (could be same server as Rust service or separate)

## Open Questions

- Will the Rust service be deployed to the same domain as the web app, or separately? (Affects CORS configuration): Answer: Same domain, same server.
- Is session persistence required across browser refreshes?: Answer: no persistence is required.
- What is the target browser support level?: Answer: Default
- Should the web app be self-hosted or served by the Rust service?: Answer: Self-hosted.
