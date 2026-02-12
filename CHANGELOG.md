# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [2026-02-12]

### Changed
- **Web interface** - Redesigned with simpler, faster static HTML pages
- **User greeting** - Updated page title from "User Info" to "Greetings!"

### Security
- Improved database security with dedicated user privileges

## [2026-02-11]

### Changed
- **User profile display** - Now shows formatted text summary instead of raw data
  - **BREAKING**: `/api/users/{user_id}` returns `text/plain` instead of JSON

## [2026-02-10]

### Added
- **REST API** - Create accounts, login, and view user profiles
  - `POST /api/users` - Create new user
  - `POST /api/login` - Authenticate user
  - `GET /api/users/{id}` - Get user profile
- **Database** - MySQL storage for user accounts and profiles

## [2025-07-11]

### Added
- **Web interface** - Initial web client for user management
