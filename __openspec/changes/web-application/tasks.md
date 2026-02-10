# Tasks: Initial Explorer Implementation

## Phase 1: Database & Backend (Rust/MySQL)
Set up the core data layer and the recursive API logic.

- [ ] **1.1 Database Migration**
  - Create `migrations/20260210_init_schema.sql`.
  - Define `users` table with email and password_hash.
  - Define `items` table with self-referencing `parent_id` (FOREIGN KEY).
- [ ] **1.2 Auth Layer (Rust)**
  - Implement `/auth/register` and `/auth/login`.
  - Use `argon2` for password hashing and `jsonwebtoken` for session management.
- [ ] **1.3 Explorer API Handlers**
  - Create `GET /api/items/:parent_id` (Fetch children where `parent_id` matches).
  - Create `POST /api/items` (Create new folder or file).
  - Implement recursive delete logic in Rust using a transaction to ensure database integrity.

## Phase 2: Core UI Components (Next.js)
Build the "Windows Explorer" style interface.

- [ ] **2.1 Global Layout & Sidebar**
  - Create a persistent sidebar that lists the "Root" level folders.
  - Implement a recursive `<SidebarItem />` component that expands to show sub-folders.
- [ ] **2.2 Main Explorer View**
  - Build a grid view that toggles icons based on `is_folder`.
  - Implement "Breadcrumb" navigation (e.g., `Home > Docs > Exercises`).
- [ ] **2.3 Item Interactivity**
  - Implement "Double Click" to enter a folder.
  - Create a `FileEditor` modal that opens when a file is clicked, allowing users to edit `content` and `description`.

## Phase 3: Wiring & Validation
Connect the frontend to the Rust backend.

- [ ] **3.1 API Integration**
  - Hook up the Next.js `fetch` calls to the Axum backend.
  - Implement "Loading" states and "Empty Folder" illustrations.
- [ ] **3.2 OpenSpec Archive**
  - Run `openspec validate web-application`.
  - Confirm all requirements in `spec.md` are met by the implementation.
  - Run `openspec archive web-application` to merge these changes into the main project specs.