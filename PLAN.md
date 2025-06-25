# TodoCLI TUI Application Plan

## Project Overview
A terminal-based todo application with a rich TUI interface, using Tokyo Night theme and GDBM for persistence.

## Architecture

### Core Components

1. **Data Layer**
   - `Todo` struct with 5 fields
   - GDBM database interface
   - CRUD operations
   - Data serialization/deserialization

2. **UI Layer**
   - Main list view
   - Detail view (view/edit)
   - Confirmation dialogs
   - Tokyo Night color scheme

3. **Application Layer**
   - Event handling
   - State management
   - Navigation logic

### Dependencies
- `ratatui` - TUI framework
- `crossterm` - Terminal handling
- `serde` - Serialization
- `chrono` - Date/time handling
- `gdbm-sys` or `gdbm` - Database
- `dirs` - Home directory detection
- `uuid` - Unique IDs for todos

## Data Model

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub subject: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub last_modified_at: DateTime<Utc>,
}
```

## UI Design

### Main View
```
┌─ TodoCLI ─────────────────────────────────────────────┐
│ 📝 Active Todos                                       │
├───────────────────────────────────────────────────────┤
│ ⭐ Fix bug in authentication                          │
│ 📚 Read Rust book chapter 10                         │
│ 🔴 Completed: Write documentation (completed)        │
├───────────────────────────────────────────────────────┤
│ 💡 Controls: Enter=View, d=Toggle, n=New, x=Delete   │
└───────────────────────────────────────────────────────┘
```

### Detail View
```
┌─ Todo Details ────────────────────────────────────────┐
│ Subject: [Fix bug in authentication              ]    │
│ Description:                                          │
│ ┌─────────────────────────────────────────────────┐   │
│ │ The login system is not properly validating    │   │
│ │ user credentials...                             │   │
│ └─────────────────────────────────────────────────┘   │
│                                                       │
│ Created: 2024-01-15 10:30:00                         │
│ Modified: 2024-01-15 14:20:00                        │
│ Status: Active                                        │
├───────────────────────────────────────────────────────┤
│ 💾 Save & Exit: Ctrl+S | Cancel: Esc                │
└───────────────────────────────────────────────────────┘
```

## Key Bindings

### Main View
- `↑/↓` or `j/k`: Navigate todos
- `Enter`: Open detail view
- `d`: Toggle completed/incomplete
- `n`: Create new todo
- `x`: Delete todo (with confirmation)
- `q`: Quit application

### Detail View
- `Tab`: Switch between fields
- `Enter`: New line in description
- `Ctrl+S`: Save and return
- `Esc`: Cancel and return

### Confirmation Dialog
- `y`: Confirm
- `n` or `Esc`: Cancel

## Color Scheme (Tokyo Night)
- Background: #1a1b26
- Foreground: #c0caf5
- Active item: #7aa2f7
- Completed item: #f7768e (red)
- Border: #414868
- Accent: #bb9af7

## Implementation Phases

### Phase 1: Core Structure
- [x] Project setup
- [ ] Basic data structures
- [ ] GDBM database integration
- [ ] Basic TUI framework setup

### Phase 2: Basic UI
- [ ] Main list view
- [ ] Basic navigation
- [ ] Tokyo Night theme implementation

### Phase 3: CRUD Operations
- [ ] Add new todos
- [ ] Toggle completion status
- [ ] Basic detail view

### Phase 4: Advanced Features
- [ ] Full detail view with editing
- [ ] Delete with confirmation
- [ ] Emoji integration
- [ ] Polish and error handling

### Phase 5: Testing & Polish
- [ ] Error handling
- [ ] Edge cases
- [ ] Performance optimization
- [ ] Documentation

## File Structure
```
src/
├── main.rs              # Entry point
├── app.rs               # Main application state
├── ui/
│   ├── mod.rs
│   ├── main_view.rs     # Todo list view
│   ├── detail_view.rs   # Todo detail/edit view
│   ├── dialog.rs        # Confirmation dialogs
│   └── theme.rs         # Tokyo Night colors
├── data/
│   ├── mod.rs
│   ├── todo.rs          # Todo struct and methods
│   └── database.rs      # GDBM operations
└── events.rs            # Event handling
```