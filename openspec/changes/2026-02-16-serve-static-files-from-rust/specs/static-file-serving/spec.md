# Static File Serving Specification

## 1. Requirements for Backend Serving Static Files
- Actix-web should be configured to serve static files from the specified directory.
- Ensure proper MIME types are set for various file extensions.

## 2. API Route Precedence
- Define rules for API route precedence over static file routes to avoid conflicts.
- Static file serving should only occur when no API routes match.

## 3. Caching Headers
- Implement caching headers to optimize static file serving.
- Specify `Cache-Control` and `Expires` headers where appropriate.

## 4. Relative URLs in Forms
- Ensure that all forms use relative URLs to maintain flexibility across environments.

## 5. Single Command Startup
- Provide a command in the documentation that allows the application to start with all configurations loaded.

## 6. Working Directory Requirements
- Specify that the application needs to run in a specific working directory to locate static files correctly.