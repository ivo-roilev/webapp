# Spec: CSS System Preference Theming

## Purpose

This spec defines requirements for CSS-only theme management based on OS/browser system preferences, eliminating the need for JavaScript theme toggles and localStorage persistence.

## Requirements

### Requirement: Theme SHALL Be Determined by System Preference

The web UI SHALL use CSS media queries to detect and apply the user's OS/browser system preference for light or dark theme, without any JavaScript or manual toggle.

#### Scenario: Dark theme applied when system prefers dark
- **WHEN** user's OS is set to dark mode
- **AND** user loads any HTML page
- **THEN** page displays with dark theme colors

#### Scenario: Light theme applied when system prefers light
- **WHEN** user's OS is set to light mode
- **AND** user loads any HTML page
- **THEN** page displays with light theme colors

#### Scenario: Theme changes when system preference changes
- **WHEN** user changes their OS theme preference while page is loaded
- **THEN** page theme updates automatically without page reload

### Requirement: All Theme Toggle UI Elements SHALL Be Removed

All theme toggle buttons, icons, and related UI elements SHALL be removed from all HTML files.

#### Scenario: No theme toggle button in login page
- **WHEN** user views login page HTML
- **THEN** no theme toggle button element exists

#### Scenario: No theme toggle button in create-user page
- **WHEN** user views create-user page HTML
- **THEN** no theme toggle button element exists

#### Scenario: No theme toggle button in user-info page
- **WHEN** user views user-info page HTML
- **THEN** no theme toggle button element exists

### Requirement: All Theme Toggle JavaScript SHALL Be Removed

All JavaScript code related to theme toggling, localStorage theme persistence, and theme icon updates SHALL be removed from all HTML files.

#### Scenario: No theme toggle JavaScript in login page
- **WHEN** user views login page source
- **THEN** no JavaScript functions for theme toggle exist
- **AND** no localStorage.getItem/setItem calls for theme exist

#### Scenario: No theme toggle JavaScript in create-user page
- **WHEN** user views create-user page source
- **THEN** no JavaScript functions for theme toggle exist
- **AND** no localStorage.getItem/setItem calls for theme exist

#### Scenario: No theme toggle JavaScript in user-info page
- **WHEN** user views user-info page source
- **THEN** no JavaScript functions for theme toggle exist
- **AND** no localStorage.getItem/setItem calls for theme exist

### Requirement: CSS SHALL Define Theme Colors Using Media Queries

The style.css file SHALL define theme colors using `@media (prefers-color-scheme: dark)` and `@media (prefers-color-scheme: light)` media queries.

#### Scenario: Dark theme colors defined
- **WHEN** style.css is parsed
- **THEN** `@media (prefers-color-scheme: dark)` block exists
- **AND** dark theme CSS variables are defined inside the block

#### Scenario: Light theme colors defined
- **WHEN** style.css is parsed
- **THEN** `@media (prefers-color-scheme: light)` block exists
- **AND** light theme CSS variables are defined inside the block

#### Scenario: All color variables covered in both themes
- **WHEN** both media query blocks are defined
- **THEN** same CSS custom properties exist in both dark and light theme blocks

### Requirement: No localStorage SHALL Be Used for Theme Persistence

No code SHALL read or write theme preference to localStorage.

#### Scenario: No theme in localStorage on page load
- **WHEN** user loads any page
- **THEN** no localStorage.getItem('theme') calls are made

#### Scenario: No theme saved to localStorage on any action
- **WHEN** user interacts with the page
- **THEN** no localStorage.setItem('theme', ...) calls are made
