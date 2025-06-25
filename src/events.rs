use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;
use crate::ui::DetailMode;

pub enum AppEvent {
    Key(KeyEvent),
    Tick,
}

pub struct EventHandler;

impl EventHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn next(&self) -> Result<AppEvent, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) => Ok(AppEvent::Key(key_event)),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}

pub fn handle_key_event(app: &mut crate::app::App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    use crate::app::AppState;

    match app.state {
        AppState::Main => handle_main_keys(app, key)?,
        AppState::Detail => handle_detail_keys(app, key)?,
        AppState::Confirm => handle_confirm_keys(app, key)?,
    }

    Ok(())
}

fn handle_main_keys(app: &mut crate::app::App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    let todos = app.get_current_todos();
    let len = todos.len();

    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('j') | KeyCode::Down => app.main_view.next(len),
        KeyCode::Char('k') | KeyCode::Up => app.main_view.previous(len),
        KeyCode::Enter => app.open_detail_view(),
        KeyCode::Char('d') => app.toggle_selected_todo()?,
        KeyCode::Char('n') => app.open_new_todo(),
        KeyCode::Char('x') => app.confirm_delete_selected(),
        KeyCode::Char('e') => app.open_edit_view(),
        _ => {}
    }

    Ok(())
}

fn handle_detail_keys(app: &mut crate::app::App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(detail_view) = &mut app.detail_view {
        match detail_view.mode {
            DetailMode::View => {
                match key.code {
                    KeyCode::Esc => app.close_detail_view_with_save()?,
                    KeyCode::Char('e') => {
                        detail_view.mode = DetailMode::Edit;
                    }
                    _ => {}
                }
            }
            DetailMode::Edit | DetailMode::New => {
                match key.code {
                    KeyCode::Esc => app.close_detail_view_with_save()?,
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.save_current_todo()?;
                    }
                    KeyCode::Tab => detail_view.next_field(),
                    KeyCode::BackTab => detail_view.previous_field(),
                    KeyCode::Char(c) => detail_view.add_char(c),
                    KeyCode::Backspace => detail_view.delete_char(),
                    KeyCode::Enter if detail_view.current_field == 1 => detail_view.add_char('\n'),
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn handle_confirm_keys(app: &mut crate::app::App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('y') => app.delete_confirmed_todo()?,
        KeyCode::Char('n') | KeyCode::Esc => app.close_confirm_dialog(),
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, AppState};
    use crate::data::{Database, Todo};
    use crate::ui::{MainView, DetailMode};

    fn create_test_app() -> App {
        let database = Database::new_in_memory().unwrap();
        App {
            state: AppState::Main,
            main_view: MainView::new(),
            detail_view: None,
            confirm_dialog: None,
            database,
            should_quit: false,
            current_todo_id: None,
            pending_delete_id: None,
        }
    }

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn create_key_event_with_modifiers(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent {
        KeyEvent::new(code, modifiers)
    }

    #[test]
    fn test_event_handler_creation() {
        let handler = EventHandler::new();
        // Just test that it can be created
        assert_eq!(std::mem::size_of_val(&handler), 0); // Zero-sized struct
    }

    #[test]
    fn test_main_keys_quit() {
        let mut app = create_test_app();
        let key = create_key_event(KeyCode::Char('q'));
        
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(app.should_quit);
    }

    #[test]
    fn test_main_keys_navigation() {
        let mut app = create_test_app();
        
        // Add some todos for navigation
        let todo1 = Todo::new("Todo 1".to_string(), "Description 1".to_string());
        let todo2 = Todo::new("Todo 2".to_string(), "Description 2".to_string());
        app.database.insert_todo_for_test(todo1);
        app.database.insert_todo_for_test(todo2);
        
        // Test down navigation
        let key = create_key_event(KeyCode::Char('j'));
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test up navigation
        let key = create_key_event(KeyCode::Char('k'));
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test arrow keys
        let key = create_key_event(KeyCode::Down);
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        
        let key = create_key_event(KeyCode::Up);
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_main_keys_open_detail_view() {
        let mut app = create_test_app();
        
        // Add a todo
        let todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        app.database.insert_todo_for_test(todo);
        
        let key = create_key_event(KeyCode::Enter);
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Detail));
        assert!(app.detail_view.is_some());
    }

    #[test]
    fn test_main_keys_new_todo() {
        let mut app = create_test_app();
        
        let key = create_key_event(KeyCode::Char('n'));
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Detail));
        assert!(app.detail_view.is_some());
        
        let detail_view = app.detail_view.as_ref().unwrap();
        assert!(matches!(detail_view.mode, DetailMode::New));
    }

    #[test]
    fn test_main_keys_edit_todo() {
        let mut app = create_test_app();
        
        // Add a todo
        let todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        app.database.insert_todo_for_test(todo);
        
        let key = create_key_event(KeyCode::Char('e'));
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Detail));
        assert!(app.detail_view.is_some());
        
        let detail_view = app.detail_view.as_ref().unwrap();
        assert!(matches!(detail_view.mode, DetailMode::Edit));
    }

    #[test]
    fn test_main_keys_confirm_delete() {
        let mut app = create_test_app();
        
        // Add a todo
        let todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        app.database.insert_todo_for_test(todo);
        
        let key = create_key_event(KeyCode::Char('x'));
        let result = handle_main_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Confirm));
        assert!(app.confirm_dialog.is_some());
    }

    #[test]
    fn test_detail_keys_view_mode() {
        let mut app = create_test_app();
        app.open_new_todo();
        
        // Set to view mode
        if let Some(detail_view) = &mut app.detail_view {
            detail_view.mode = DetailMode::View;
        }
        
        // Test escape key
        let key = create_key_event(KeyCode::Esc);
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test edit key
        app.open_new_todo();
        if let Some(detail_view) = &mut app.detail_view {
            detail_view.mode = DetailMode::View;
        }
        
        let key = create_key_event(KeyCode::Char('e'));
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
        
        if let Some(detail_view) = &app.detail_view {
            assert!(matches!(detail_view.mode, DetailMode::Edit));
        }
    }

    #[test]
    fn test_detail_keys_edit_mode() {
        let mut app = create_test_app();
        app.open_new_todo();
        
        // Test tab navigation
        let key = create_key_event(KeyCode::Tab);
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test character input
        let key = create_key_event(KeyCode::Char('H'));
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test backspace
        let key = create_key_event(KeyCode::Backspace);
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
        
        // Test Ctrl+S save
        let key = create_key_event_with_modifiers(KeyCode::Char('s'), KeyModifiers::CONTROL);
        let result = handle_detail_keys(&mut app, key);
        assert!(result.is_ok());
    }

    #[test]
    fn test_confirm_keys() {
        let mut app = create_test_app();
        
        // Set up confirm dialog
        let todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        app.database.insert_todo_for_test(todo);
        app.confirm_delete_selected();
        
        // Test 'n' key (cancel)
        let key = create_key_event(KeyCode::Char('n'));
        let result = handle_confirm_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Main));
        assert!(app.confirm_dialog.is_none());
        
        // Set up confirm dialog again
        app.confirm_delete_selected();
        
        // Test Esc key (cancel)
        let key = create_key_event(KeyCode::Esc);
        let result = handle_confirm_keys(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Main));
        assert!(app.confirm_dialog.is_none());
    }

    #[test]
    fn test_handle_key_event_routing() {
        let mut app = create_test_app();
        
        // Test main state routing
        let key = create_key_event(KeyCode::Char('q'));
        let result = handle_key_event(&mut app, key);
        assert!(result.is_ok());
        assert!(app.should_quit);
        
        // Reset app
        app = create_test_app();
        app.open_new_todo();
        
        // Test detail state routing
        let key = create_key_event(KeyCode::Char('H'));
        let result = handle_key_event(&mut app, key);
        assert!(result.is_ok());
        
        // Test confirm state routing
        app = create_test_app();
        let todo = Todo::new("Test Todo".to_string(), "Description".to_string());
        app.database.insert_todo_for_test(todo);
        app.confirm_delete_selected();
        
        let key = create_key_event(KeyCode::Char('n'));
        let result = handle_key_event(&mut app, key);
        assert!(result.is_ok());
        assert!(matches!(app.state, AppState::Main));
    }
}