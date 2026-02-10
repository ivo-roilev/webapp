# Change: Implement Hierarchical File Explorer

## Why
Users need a way to organize their data using a familiar "Windows Explorer" mental model with folders, sub-folders, and files.

## What Changes
- **Database**: Add `users` and `items` tables with self-referencing parent IDs.
- **Auth**: Implement user registration and login endpoints in Rust.
- **API**: Create `GET /items/:parent_id` to list folder contents.
- **UI**: Build a recursive sidebar and a main grid view for files.

## Impact
- **Affected Specs**: `auth-spec`, `explorer-spec`
- **Affected Code**: `backend/src/models/`, `frontend/src/components/Explorer/`

## Approach
1. Set up MySQL migrations for the `items` table.
2. Implement recursive delete logic in Rust (deleting a folder must delete all children).
3. Build the Next.js frontend to toggle between "Folder" and "File" icons based on the `is_folder` flag.