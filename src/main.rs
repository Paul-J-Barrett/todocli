mod app;
mod data;
mod events;
mod ui;

use app::{App, AppState};
use data::Todo;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::{AppEvent, EventHandler};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new()?;
    let event_handler = EventHandler::new();

    // Main loop
    let result = run_app(&mut terminal, &mut app, &event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    event_handler: &EventHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            
            match app.state.clone() {
                AppState::Main => {
                    let todos = app.get_current_todos();
                    let todo_refs: Vec<&Todo> = todos.iter().collect();
                    app.main_view.render(frame, area, &todo_refs);
                }
                AppState::Detail => {
                    let todos = app.get_current_todos();
                    let todo_refs: Vec<&Todo> = todos.iter().collect();
                    app.main_view.render(frame, area, &todo_refs);
                    
                    if let Some(detail_view) = &app.detail_view {
                        detail_view.render(frame, area);
                    }
                }
                AppState::Confirm => {
                    let todos = app.get_current_todos();
                    let todo_refs: Vec<&Todo> = todos.iter().collect();
                    app.main_view.render(frame, area, &todo_refs);
                    
                    if let Some(confirm_dialog) = &app.confirm_dialog {
                        confirm_dialog.render(frame, area);
                    }
                }
            }
        })?;

        match event_handler.next()? {
            AppEvent::Key(key) => {
                events::handle_key_event(app, key)?;
            }
            AppEvent::Tick => {
                // Handle periodic updates if needed
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
