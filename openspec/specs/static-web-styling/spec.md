## Overview

A single CSS stylesheet providing professional, responsive design for all three static HTML pages (login.html, create-user.html, user-info.html). The stylesheet establishes visual consistency, typography, color scheme, layout, and responsive behavior across the entire application.

## Design System

### Color Palette

**Light Mode (Default)**
- **Primary**: #3498db (blue) - buttons, links, accents
- **Background**: #f5f5f5 (light gray) - page backgrounds
- **Surface**: #ffffff (white) - cards, form inputs
- **Text**: #2c3e50 (dark blue-gray) - body text
- **Text Secondary**: #666 (gray) - secondary text
- **Error**: #e74c3c (red) - error messages
- **Border**: #ddd (light gray) - form input borders
- **Footer**: #2c3e50 (dark blue-gray) - footer background

**Dark Mode**
- **Primary**: #5dade2 (lighter blue) - buttons, links, accents
- **Background**: #1a1a1a (very dark) - page backgrounds
- **Surface**: #2d2d2d (dark gray) - cards, form inputs
- **Text**: #e0e0e0 (light gray) - body text
- **Text Secondary**: #b0b0b0 (medium gray) - secondary text
- **Error**: #ff6b6b (lighter red) - error messages
- **Border**: #444 (dark gray) - form input borders
- **Footer**: #0d0d0d (almost black) - footer background

### Typography
- **Font Family**: System fonts (-apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica Neue, Arial)
- **Base Size**: 1rem (16px)
- **Headings**: h1 2rem, bold, dark color
- **Labels**: 0.95rem, medium weight
- **Body**: 1rem, regular weight
- **Small**: 0.875rem (for errors, captions)

### Layout
- **Container max-width**: 1200px
- **Page padding**: 2rem for desktop, 1rem for mobile
- **Form spacing**: 1.5rem between form groups
- **Button height**: ~48px (accessible touch target)
- **Input height**: ~36px

### Material & Spacing
- **Border radius**: 4px (cards, inputs, buttons)
- **Box shadow**: Light shadow on cards (0 2px 8px rgba(0,0,0,0.1))
- **Padding**: Consistent spacing: 0.75rem inputs, 2rem page containers
- **Gap**: 1rem between buttons, 1.5rem between form sections

## Component Specifications

### Theme Toggle Button
- Small button positioned in top-right corner of page
- Displays as icon or text: "🌙" for dark mode / "☀️" for light mode
- Non-intrusive design, ~40px square
- Toggles between light and dark mode
- Preference stored in localStorage using key `theme` (value: 'light' or 'dark')
- Applied to root element via `data-theme` attribute for CSS variable switching
- Visible on all three pages (login, create-user, user-info)
- Accessible: proper contrast in both light and dark modes

### Page Links
- Simple text links or buttons for page transitions (no global navigation bar)
- Blue text color (#3498db) for links, underline on hover
- Styled as inline links or secondary buttons depending on context
- Links: Hover effect darkens color
- Used for: "Create New User" link on login page, "Back to Login" link on create-user page
- User-info page has NO navigation links to other pages (only Logout and Retry buttons)

### Form Components
- Label above input
- Input with light gray border, focus state: blue border + light blue background
- Error messages: Red text below field
- Validation: Inline, clear feedback on invalid fields
- Button: Full-width or auto-width in form

### Buttons
- **Primary Button**: Blue background, white text, hover darkens
- **Secondary Button**: Gray background, white text, hover darkens
- **Disabled State**: Reduced opacity (0.6), not-allowed cursor
- **Loading State**: Button text changes (e.g., "Logging in..."), appears disabled
- **Font Weight**: 500 (medium)
- **Padding**: 0.75rem 1.5rem

### Cards/Pages
- White background, light shadow
- Padding: 2rem
- Border radius: 8px
- Desktop: Centered in viewport with max-width 600px
- Mobile: Full width with padding

### Error/Alert Messages
- Red text color (#e74c3c)
- Smaller font (0.875rem)
- Display inline below fields or as block message
- Visible when validation fails or API error occurs

### Footer
- Dark background (#2c3e50), white text
- Fixed at bottom or pushed down with flexbox
- Small font size (0.875rem)
- Centered text
- Full width

### Responsive Behavior
- **Mobile** (< 600px):
  - Single column layout
  - Full-width buttons
  - Reduced padding and margins
  - Larger touch targets (48px min height)

- **Desktop** (600px+):
  - Centered container with max-width 600px
- All pages readable and usable on:
  - Mobile: 320px width minimum
  - Tablet: 600px width
  - Desktop: 1200px+ width

### Dark Mode Implementation
- Use CSS custom properties (variables) for theming: `--bg-primary`, `--text-primary`, etc.
- Root element has `data-theme="light"` or `data-theme="dark"` attribute
- Dark mode colors override light mode colors when data-theme changes
- Smooth color transitions on theme toggle (consider brief transition for better UX)
- System preference respected: detect `prefers-color-scheme` media query for initial theme
- Toggle button updates localStorage and triggers page re-render with new theme

### Media Queries
- Base styles for mobile
- `@media (max-width: 600px)`: Mobile-specific overrides
- `@media (min-width: 600px)`: Desktop-specific layout
- `@media (prefers-color-scheme: dark)`: Initial dark mode preference detection

## No Requirements
- System-level dark mode sync (one-time preference detection only)
- Animation or transitions (beyond hover states and subtle theme transitions)
- CSS frameworks or preprocessors
- CSS-in-JS
- Advanced accessibility features (ARIA, screen reader specific styling)
- Print stylesheets
- Internationalization (RTL language support)
