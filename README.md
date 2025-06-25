# TodoCLI

A beautiful terminal-based todo application built with Rust, featuring a rich TUI interface with Tokyo Night theme.

## Features

- 🎨 **Tokyo Night Theme** - Beautiful dark color scheme
- ⌨️ **Keyboard-Driven** - Efficient navigation with vim-like controls
- 📝 **Rich Todo Management** - Subject, description, timestamps
- 🔴 **Visual Status** - Red color for completed items
- 💾 **Persistent Storage** - Binary GDBM-style database storage
- 🎯 **Modal Interface** - Detail views for editing and viewing

## Installation

```bash
git clone <repository-url>
cd todocli
cargo build --release
cargo run
```

## Usage

### Main View Controls
- `j/k` or `↑/↓` - Navigate todos
- `Enter` - Open detail view
- `d` - Toggle completed/incomplete
- `n` - Create new todo
- `e` - Edit selected todo
- `x` - Delete todo (with confirmation)
- `t` - Toggle between active/all todos view
- `q` - Quit application

### Detail View Controls
- `Tab` - Switch between fields
- `Ctrl+S` - Save and return
- `Esc` - Cancel and return
- `e` - Switch to edit mode (from view mode)

### Confirmation Dialog
- `y` - Confirm action
- `n` or `Esc` - Cancel action

## Data Storage

Todos are stored in `~/.config/todo/todo.gdbm` as a binary database file. The data structure contains:

```json
{
  "todo-id": {
    "id": "unique-uuid",
    "subject": "Todo title",
    "description": "Detailed description",
    "created_at": "2024-01-01T10:00:00Z",
    "closed_at": null,
    "last_modified_at": "2024-01-01T10:00:00Z"
  }
}
```

## Todo Fields

Each todo contains 5 fields:
- **Subject**: Brief title/summary
- **Description**: Detailed description (supports multiline)
- **Created_at**: When the todo was created
- **Closed_at**: When the todo was completed (null if active)
- **Last_modified_at**: When the todo was last updated

## Development

See [PLAN.md](PLAN.md) for detailed implementation plan and architecture.

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
```bash
cargo test
cargo clippy
cargo fmt
```

## License

[Add your license here]

## Contributing

[Add contributing guidelines here]