## Implementation Tasks

### Phase 1: Create Core HTML Pages

#### Task 1.1: Create login.html
- Create `src/web/login.html` as the main landing/index page
- Include form with username and password fields
- Form validation: username and password required
- Form submission handler:
  - Validate fields
  - Send POST request to `/api/login`
  - On success: store user_id in localStorage, redirect to `user-info.html`
  - On error: display error message
- Include "Create New User" link to `create-user.html`
- Include theme toggle button (🌙/☀️) in top-right corner
- No global navigation bar
- Responsive design following `static-web-styling` spec

**Dependencies**: static-web-styling spec
**Acceptance Criteria**:
- Form renders correctly on desktop and mobile
- Validation works for empty fields
- API calls succeed and redirect works
- Error handling displays messages

#### Task 1.2: Create create-user.html
- Create `src/web/create-user.html`
- Include form with:
  - Required fields: username, password
  - Optional fields: first name, last name, email, title, hobby
- Form validation: username and password required
- Form submission handler:
  - Validate required fields only
  - Send POST request to `/api/users` with provided data (omit empty optional fields)
  - On success: store user_id in localStorage, redirect to `user-info.html`
  - On error: display error message
- Include "Back to Login" link to `login.html`
- Include theme toggle button (🌙/☀️) in top-right corner
- No global navigation bar
- Responsive design following `static-web-styling` spec

**Dependencies**: static-web-styling spec
**Acceptance Criteria**:
- Form renders all 7 fields correctly
- Optional fields are truly optional
- API call includes only non-empty fields
- Error handling displays messages
- Mobile layout is readable

#### Task 1.3: Create user-info.html
- Create `src/web/user-info.html`
- On page load:
  - Check for user_id in localStorage
  - If missing: automatically redirect to login.html
  - If present: fetch greeting from `/api/users/{user_id}` via POST request
- Display loading state while fetching
- On success: display greeting message prominently
- On error: display error message with "Retry" button
- Include "Logout" button that clears localStorage and redirects to login
- Include theme toggle button (🌙/☀️) in top-right corner
- NO navigation links between pages
- Responsive design following `static-web-styling` spec

**Dependencies**: static-web-styling spec
**Acceptance Criteria**:
- Auto-redirect to login works when no user_id in localStorage
- API call fetch works with correct endpoint
- All states (loading, success, error) render correctly
- Logout button clears localStorage and redirects
- Mobile layout is readable

### Phase 2: Create Stylesheet

#### Task 2.1: Create style.css
- Create `src/web/style.css` implementing the `static-web-styling` specification
- Include:
  - CSS custom properties (variables) for light and dark mode colors
  - Base styles (*, body, fonts)
  - Theme toggle button styles (top-right corner, icon-based)
  - Page link styles (simple inline links)
  - Form component styles (labels, inputs, buttons, errors)
  - Page/card styles
  - Footer styles
  - Dark mode color overrides when `data-theme="dark"` is set
  - Responsive breakpoint at 600px
  - Color palette and typography as specified
- Test on multiple browsers:
  - Chrome 90+
  - Safari 14+
  - Firefox 88+
- Test on multiple screen sizes:
  - Mobile 320px
  - Tablet 768px
  - Desktop 1200px+

**Dependencies**: static-web-styling spec
**Acceptance Criteria**:
- All pages render with consistent styling
- Page links are blue and change on hover
- Forms are accessible with proper spacing
- Mobile layout stacks appropriately
- All colors match spec in light mode
- All colors match spec in dark mode
- Theme toggle button visible on all pages
- Professional appearance in both light and dark modes

### Phase 3: Update Server/Deployment

#### Task 3.1: Update web server configuration
- Ensure web server (e.g., nginx, Express, simple HTTP server) serves from `src/web/` directory
- Verify CORS headers allow API calls to backend (if on different port)
- Document API base URL configuration (currently `http://localhost:8000`)
- Setup development server for local testing

**Acceptance Criteria**:
- Static files served correctly from src/web/
- index.html or login.html is default page
- Links between pages work
- CSS and form functionality work

#### Task 3.2: Configure API base URL
- Define method for configuring API base URL (currently hardcoded to `http://localhost:8000`)
- Options: environment variable, config file, or server-side injection
- Document configuration for deployment
- Test in multiple environments (localhost, staging, production)

**Acceptance Criteria**:
- API calls reach correct endpoint
- Configuration method is documented
- Works in all environments

### Phase 4: Testing & Validation

#### Task 4.1: Test login flow
- Test with valid credentials
- Test with invalid credentials
- Test form validation (empty fields)
- Test localStorage persistence across page reload
- Test error messages display correctly

**Acceptance Criteria**:
- All login scenarios work
- User can log in, navigate to user-info, close browser, return and still be logged in

#### Task 4.2: Test user creation flow
- Test with all fields filled
- Test with only required fields
- Test form validation (empty username/password)
- Test error handling for duplicate username
- Test redirect to user-info after success

**Acceptance Criteria**:
- All user creation scenarios work
- Optional fields are truly optional
- Error messages are clear

#### Task 4.3: Test user-info display
- Test with valid user_id and greeting displays
- Test with no session: auto-redirect to login works
- Test error handling when API fails
- Test logout functionality
- Test retry button on error

**Acceptance Criteria**:
- Greeting displays correctly
- No-session state handled properly
- Logout clears session
- Retry functionality works

#### Task 4.4: Theme toggle and dark mode testing
- Test light mode theme displays correctly on all pages
- Test dark mode theme displays correctly on all pages
- Test toggle button works on all pages
- Test localStorage persistence of theme preference across page reloads
- Test contrast and readability in both themes
- Test on mobile and desktop layouts
- Verify system preference (prefers-color-scheme) is detected initially

#### Task 4.5: Responsive design testing
- Test on mobile (320px), tablet (768px), desktop (1200px+)
- Test on Chrome, Safari, Firefox
- Test form usability and readability
- Test touch targets are adequate (48px minimum)

**Acceptance Criteria**:
- All pages readable and usable on all screen sizes
- Text is readable
- Forms are easy to interact with
- Navigation is accessible

#### Task 4.6: Performance validation
- Verify page load time is fast
- Verify no console errors
- Verify API calls complete successfully
- Check file sizes are reasonable

**Acceptance Criteria**:
- Each HTML file < 5KB
- CSS file < 10KB
- Total payload < 20KB
- No JavaScript errors

### Phase 5: Documentation & Cleanup

#### Task 5.1: Add theme toggle JavaScript
- Create or add theme toggle functionality to each HTML page
- On page load:
  - Check localStorage for `theme` preference
  - If no preference, detect system preference using `window.matchMedia('(prefers-color-scheme: dark)')`
  - Apply theme by setting `document.documentElement.setAttribute('data-theme', theme)`
- Toggle button click handler:
  - Switch between light and dark themes
  - Update localStorage with new preference
  - Update HTML element's data-theme attribute
- Keep code minimal and inline for simple static pages

**Acceptance Criteria**:
- Theme persists across page reloads
- System preference detected on first visit
- Toggle button functions on all pages
- No console errors

#### Task 5.2: Document minimum browser requirements
- Create/update README in src/web/ or project root
- Document minimum versions:
  - Chrome 90+
  - Safari 14+
  - Firefox 88+
- Document local development setup
- Document API configuration

**Acceptance Criteria**:
- Clear documentation on setup
- Browser requirements documented
- API base URL configuration documented

#### Task 5.3: Cleanup old React files
- Remove React/TypeScript files:
  - Delete src/web/src/ (all React components)
  - Delete package.json, tsconfig*, vite.config.ts
  - Delete public/ folder if empty
  - Delete dist/, build/, node_modules/
- Verify src/web/ contains only:
  - index.html (or served from login.html)
  - login.html
  - create-user.html
  - user-info.html
  - style.css

**Acceptance Criteria**:
- Old React files removed
- Only 5 files remain in src/web/
- No references to Vite, TypeScript, or React

#### Task 5.3: Git commit and documentation
- Commit the working implementation
- Create summary of changes
- Document in project CHANGELOG
- Review specifications to ensure all requirements met

**Acceptance Criteria**:
- Clean git history
- All specifications implemented
- Change completed and ready for merge

## Execution Order

1. **Task 2.1** (Create style.css) - needed by all pages
2. **Task 1.1, 1.2, 1.3** (Create HTML pages) - can be parallel, all depend on style.css
3. **Task 3.1, 3.2** (Server configuration) - needed for testing
4. **Task 4.1-4.5** (Testing) - all depend on phases 1-3
5. **Task 5.1-5.3** (Documentation & cleanup) - final phase

## Success Criteria (Overall)

- ✅ All three HTML pages created and functional
- ✅ Single CSS stylesheet implemented per spec
- ✅ Login flow works (username/password → user_id → redirect)
- ✅ User creation flow works (form → user_id → redirect)
- ✅ User info display works (fetch greeting → display → logout)
- ✅ Responsive design works on all screen sizes
- ✅ API calls use correct endpoints
- ✅ Error handling covers all scenarios
- ✅ No React/Vite/TypeScript code remains
- ✅ src/web/ contains only 5 files
- ✅ Tests pass on Chrome, Safari, Firefox
- ✅ Documentation complete
