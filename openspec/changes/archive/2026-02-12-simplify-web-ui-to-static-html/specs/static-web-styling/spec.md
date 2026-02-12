# Static Web Styling Specification

## Overview
A single comprehensive CSS stylesheet providing professional styling for all three static HTML pages with responsive design and light/dark theme support.

## File Location
`src/web/style.css`

## Design Requirements

### Theme Support
Implement dual-theme system using CSS custom properties:

#### Light Theme (Default)
- Background: Light gray (#f0f0f0)
- Card background: White (#ffffff)
- Primary text: Dark gray (#333333)
- Accent: Blue (#007bff)
- Accent hover: Darker blue (#0056b3)

#### Dark Theme
- Background: Dark gray (#1a1a1a)
- Card background: Charcoal (#2a2a2a)
- Primary text: Light gray (#e0e0e0)
- Accent: Light blue (#4dabf7)
- Accent hover: Brighter blue (#74c0fc)

### Theme Implementation
```css
:root {
  /* Light theme variables */
  --bg-primary: #f0f0f0;
  --bg-card: #ffffff;
  --text-primary: #333333;
  --accent: #007bff;
  --accent-hover: #0056b3;
}

[data-theme="dark"] {
  /* Dark theme overrides */
  --bg-primary: #1a1a1a;
  --bg-card: #2a2a2a;
  --text-primary: #e0e0e0;
  --accent: #4dabf7;
  --accent-hover: #74c0fc;
}
```

### Layout Structure

#### Page Container
- Full viewport height
- Centered content
- Background color from theme
- Smooth transitions on theme change

#### Card Component
- Centered on page
- Maximum width: 400px
- White background (light) / Charcoal background (dark)
- Rounded corners (8px border-radius)
- Box shadow for depth
- Padding: 2rem
- Responsive width on mobile

### Typography

#### Headings
- **Page Title (h1)**:
  - Font size: 1.5rem
  - Text transform: uppercase
  - Letter spacing: 0.1em
  - Color: Accent color
  - Margin: 0 0 1rem 0
  - Used for: "Login", "Create New User", "User Information"

- **Greeting Text**:
  - Font size: 2rem
  - Font weight: Bold
  - Color: Primary text color
  - Margin: 0
  - Used for: "Hello, John! Welcome back."

#### Body Text
- Font family: System font stack (Arial, Helvetica, sans-serif)
- Base size: 16px
- Line height: 1.5
- Color: Primary text from theme

### Form Elements

#### Input Fields
- Full width
- Padding: 0.75rem
- Border: 1px solid #ddd (light) / #444 (dark)
- Border radius: 4px
- Background: Theme card background
- Color: Theme primary text
- Margin: 0.5rem 0 1rem 0
- Focus state: Blue outline

#### Labels
- Display: block
- Margin bottom: 0.25rem
- Font weight: 500
- Required indicator: Red asterisk (*)

#### Buttons
- Full width primary buttons
- Padding: 0.75rem
- Background: Accent color
- Color: White
- Border: None
- Border radius: 4px
- Font weight: 600
- Cursor: pointer
- Hover state: Darker accent
- Transition: 0.2s ease
- Disabled state: Reduced opacity

### Navigation Links
- Display: inline-block
- Text align: center
- Margin: 1rem 0
- Color: Accent
- Hover: Accent hover color
- No underline

### Theme Toggle Button
- Position: Fixed top-right (1rem, 1rem)
- Size: 3rem × 3rem
- Background: Transparent
- Border: 2px solid accent
- Border radius: 50% (circular)
- Font size: 1.5rem
- Cursor: pointer
- Z-index: 1000
- Hover: Background accent with 0.1 opacity

### Error Messages
- Color: Red (#dc3545)
- Font size: 0.9rem
- Margin: 0.5rem 0

### Responsive Design

#### Mobile (< 768px)
- Card width: 90% with 1rem padding
- Smaller font sizes
- Reduced spacing

#### Tablet (768px - 1199px)
- Card max-width: 400px
- Standard font sizes

#### Desktop (≥ 1200px)
- Card max-width: 400px
- Enhanced spacing
- Larger interactive elements

### Animation and Transitions
- Theme transitions: 0.3s ease for all properties
- Button hover: 0.2s ease
- Input focus: 0.15s ease
- Smooth theme switch without flash

## Technical Requirements

### CSS Structure
```css
/* 1. CSS Reset and base styles */
/* 2. CSS Variables (light theme) */
/* 3. Dark theme overrides */
/* 4. Layout (body, container, card) */
/* 5. Typography (h1, p, labels) */
/* 6. Form elements (input, button) */
/* 7. Theme toggle button */
/* 8. Navigation links */
/* 9. Error messages */
/* 10. Responsive media queries */
```

### Browser Compatibility
- CSS custom properties (CSS variables)
- Modern flexbox layout
- CSS transitions and transforms
- Media queries
- No vendor prefixes needed for target browsers

### Performance
- Single CSS file (no imports)
- Minimal selectors depth
- No expensive properties (avoid box-shadow on scroll)
- Efficient theme switching (CSS variables)

### Accessibility
- Sufficient color contrast (WCAG AA minimum)
- Focus indicators for all interactive elements
- Readable font sizes (16px minimum)
- Touch target sizes (minimum 44×44px)

## Success Criteria
- All three pages styled consistently
- Light and dark themes both fully functional
- Smooth theme transitions with no flash
- Responsive on mobile (320px), tablet (768px), desktop (1200px+)
- Page title visually distinct from greeting text
- No unused styles (no logout button styles)
- Single stylesheet under 500 lines
- Works in Chrome 90+, Safari 14+, Firefox 88+
- Passes WCAG AA contrast requirements
- No framework dependencies (pure CSS)
