# Design: Hierarchical File Explorer

## Data Model
We will use a recursive tree structure for folders.

### Rust Structs (The "Source of Truth")
```rust
struct Folder {
    id: u64,
    name: String,
    parent_id: Option<u64>, // None means root
    user_id: u64,
}

struct File {
    id: u64,
    name: String,
    folder_id: u64,
    user_id: u64,
}