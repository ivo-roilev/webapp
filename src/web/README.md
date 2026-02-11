# Web REST Client

A React + TypeScript single-page application providing a web interface for the User Management REST API.

## Pages

- **Create User** (`/create-user`) — Register a new user with username, password, and optional profile fields
- **Login** (`/login`) — Authenticate with username and password
- **User Info** (`/user-info`) — View the profile of the currently authenticated user

## Prerequisites

- Node.js (v18+)
- The Rust REST service running on `http://127.0.0.1:8080`

## Development

```bash
cd src/web
npm install
npm run dev
```

The dev server starts on `http://localhost:3000` with API requests proxied to the Rust service.

## Production Build

```bash
npm run build
```

Output is in `dist/`. Serve it from the same domain as the Rust API service.

## Project Structure

```
src/
  components/    # Reusable UI components (Layout, FormComponents)
  pages/         # Page components (CreateUserPage, LoginPage, UserInfoPage)
  utils/         # API client and storage utilities
  types.ts       # TypeScript type definitions
```
