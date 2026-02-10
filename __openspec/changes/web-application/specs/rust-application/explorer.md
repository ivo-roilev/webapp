# Spec: Hierarchical Explorer System

## Overview
A web-based file management interface that allows users to organize items (folders and files) in a nested hierarchy. Each item is owned by a specific user and contains metadata (description) and, for files, text-based content.

---

## 1. Authentication & Security
The system **MUST** ensure data isolation between users. No user should be able to view, edit, or delete items belonging to another `user_id`.

### Scenario: Unauthorized Access
* **GIVEN** an authenticated User A and an item (ID: 500) belonging to User B
* **WHEN** User A attempts to access `GET /api/items/500`
* **THEN** the Rust backend **MUST** return a `403 Forbidden` or `404 Not Found` error.

---

## 2. Navigation & Hierarchy
The explorer uses a recursive parent-child relationship to render the UI.

### Scenario: Rendering the Root Directory
* **GIVEN** a user is logged in
* **WHEN** they navigate to the dashboard
* **THEN** the system fetches items where `parent_id` is `NULL` and `user_id` matches the session.
* **AND** the UI renders folders first, then files, sorted alphabetically.

### Scenario: Deep Navigation
* **GIVEN** a user clicks on a folder named "Workouts" (ID: 10)
* **WHEN** the route changes to `/explorer/10`
* **THEN** the system fetches items where `parent_id` is `10`.
* **AND** the UI displays a "Breadcrumb" path: `Home > Workouts`.



---

## 3. Item Management
Users can create, read, and update folders and files.

### Scenario: Creating a File
* **GIVEN** a user is inside a folder
* **WHEN** they click "New File" and enter a name and description
* **THEN** the system inserts a record into the `items` table with `is_folder = false`.
* **AND** the `content` field is initialized as an empty string.

### Scenario: Viewing/Editing File Content
* **GIVEN** an item is a file (`is_folder = false`)
* **WHEN** a user clicks the item
* **THEN** the UI opens a text area displaying the `content` and `description` fields.
* **AND** changes are persisted to the database via an `UPDATE` query.

---

## 4. Deletion Logic (Recursive)
Because this is a one-to-many hierarchy, deletion must be handled carefully.

### Scenario: Deleting a Folder
* **GIVEN** a folder contains sub-folders and files
* **WHEN** a user selects "Delete" on the parent folder
* **THEN** the Rust backend **MUST** perform a recursive deletion (or a cascaded database delete).
* **AND** all child items with that folder's ID as their `parent_id` are removed.

---

## 5. Technical Constraints
* **Database**: MySQL 8.0+
* **Backend**: Rust (Axum + SQLx)
* **Frontend**: Next.js (React)
* **API Format**: JSON