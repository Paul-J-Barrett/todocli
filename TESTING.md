# TodoCLI Testing Guide

This document describes the comprehensive testing suite implemented for the TodoCLI application using Rust's built-in testing framework.

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Specific Test Module
```bash
# Run only Todo struct tests
cargo test data::todo::tests

# Run only Database tests
cargo test data::database::tests

# Run only App tests
cargo test app::tests

# Run only UI tests
cargo test ui::

# Run only Event handling tests
cargo test events::tests
```

### Run Specific Test
```bash
cargo test test_todo_creation
```

## Test Coverage

### Data Layer Tests

#### Todo Struct (`src/data/todo.rs`)
- **test_todo_creation**: Validates todo creation with proper timestamps
- **test_todo_completion_toggle**: Tests completion status toggling and timestamp updates
- **test_todo_update**: Tests updating todo content and last_modified_at
- **test_status_icon**: Tests status icon display for different states
- **test_display_title**: Tests formatted title display
- **test_is_completed**: Tests completion status detection

#### Database (`src/data/database.rs`)
- **test_database_creation**: Tests in-memory database initialization
- **test_add_todo**: Tests adding todos to database
- **test_get_todo**: Tests retrieving todos by ID
- **test_update_todo**: Tests updating existing todos
- **test_delete_todo**: Tests removing todos from database
- **test_get_all_todos_sorting**: Tests sorting logic (active first, then by date)
- **test_get_active_todos**: Tests filtering active todos
- **test_get_completed_todos**: Tests filtering completed todos

### Application Logic Tests

#### App State Management (`src/app.rs`)
- **test_app_creation**: Tests application initialization
- **test_get_current_todos_empty**: Tests empty todo list handling
- **test_get_current_todos_with_data**: Tests todo list retrieval
- **test_get_selected_todo**: Tests selected todo retrieval
- **test_open_detail_view**: Tests opening detail view for viewing
- **test_open_edit_view**: Tests opening detail view for editing
- **test_open_new_todo**: Tests creating new todo flow
- **test_close_detail_view**: Tests closing detail view
- **test_toggle_selected_todo**: Tests todo completion toggling
- **test_confirm_delete_selected**: Tests delete confirmation dialog
- **test_close_confirm_dialog**: Tests closing confirmation dialog
- **test_quit**: Tests application quit functionality

### UI Component Tests

#### Main View (`src/ui/main_view.rs`)
- **test_main_view_creation**: Tests main view initialization
- **test_navigation_next**: Tests forward navigation through todo list
- **test_navigation_previous**: Tests backward navigation through todo list
- **test_navigation_with_no_selection**: Tests navigation edge cases
- **test_selection_state**: Tests selection state management

#### Detail View (`src/ui/detail_view.rs`)
- **test_detail_view_creation_for_viewing**: Tests view mode initialization
- **test_detail_view_creation_for_editing**: Tests edit mode initialization
- **test_detail_view_creation_for_new**: Tests new todo mode initialization
- **test_field_navigation**: Tests field navigation (subject/description)
- **test_add_char**: Tests character input to fields
- **test_delete_char**: Tests character deletion from fields
- **test_is_valid**: Tests form validation logic
- **test_completed_todo_detail_view**: Tests detail view for completed todos

### Event Handling Tests

#### Key Event Processing (`src/events.rs`)
- **test_event_handler_creation**: Tests event handler initialization
- **test_main_keys_quit**: Tests quit key handling
- **test_main_keys_navigation**: Tests navigation key handling
- **test_main_keys_open_detail_view**: Tests detail view opening
- **test_main_keys_new_todo**: Tests new todo creation
- **test_main_keys_edit_todo**: Tests edit mode activation
- **test_main_keys_confirm_delete**: Tests delete confirmation
- **test_detail_keys_view_mode**: Tests view mode key handling
- **test_detail_keys_edit_mode**: Tests edit mode key handling
- **test_confirm_keys**: Tests confirmation dialog key handling
- **test_handle_key_event_routing**: Tests event routing to correct handlers

## Test Architecture

### Test Utilities

#### In-Memory Database
- `Database::new_in_memory()`: Creates database without disk persistence
- `Database::insert_todo_for_test()`: Direct todo insertion for testing

#### Test Helpers
- `create_test_app()`: Creates app instance with in-memory database
- `create_test_todo()`: Creates todo with test data
- `create_key_event()`: Creates keyboard events for testing

### Testing Principles

1. **Isolation**: Each test is independent and doesn't affect others
2. **No Side Effects**: Tests use in-memory databases to avoid file system changes
3. **Comprehensive Coverage**: Tests cover happy paths, edge cases, and error conditions
4. **Fast Execution**: All tests run quickly without external dependencies
5. **Clear Assertions**: Each test has clear, specific assertions

### Mock Data

Tests use realistic but controlled data:
- Predictable todo IDs and content
- Controlled timestamps for testing sorting
- Various completion states for testing filters

## Test Results

When all tests pass, you should see output like:
```
running 50 tests
test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Continuous Integration

These tests are designed to run in CI environments:
- No external dependencies
- No file system requirements
- Fast execution time
- Deterministic results

## Adding New Tests

When adding new features:

1. **Add unit tests** for new functions/methods
2. **Add integration tests** for feature workflows
3. **Test edge cases** and error conditions
4. **Update this documentation** with new test descriptions

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Arrange
        let mut app = create_test_app();
        
        // Act
        let result = app.new_feature();
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(app.some_state, expected_value);
    }
}
```

## Debugging Tests

### Run with Debug Output
```bash
RUST_BACKTRACE=1 cargo test
```

### Run Single Test with Output
```bash
cargo test test_name -- --nocapture
```

### Check Test Coverage
```bash
# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

This comprehensive testing suite ensures the TodoCLI application is reliable, maintainable, and bug-free.