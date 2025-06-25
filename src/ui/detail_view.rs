use crate::data::Todo;
use crate::ui::theme::TokyoNightTheme;
use chrono::{DateTime, Utc};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

#[derive(Clone)]
pub enum DetailMode {
    View,
    Edit,
    New,
}

pub struct DetailView {
    pub mode: DetailMode,
    pub subject: String,
    pub description: String,
    pub created_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub last_modified_at: Option<DateTime<Utc>>,
    pub current_field: usize, // 0 = subject, 1 = description
}

impl DetailView {
    pub fn new_for_viewing(todo: &Todo) -> Self {
        Self {
            mode: DetailMode::View,
            subject: todo.subject.clone(),
            description: todo.description.clone(),
            created_at: Some(todo.created_at),
            closed_at: todo.closed_at,
            last_modified_at: Some(todo.last_modified_at),
            current_field: 0,
        }
    }

    pub fn new_for_editing(todo: &Todo) -> Self {
        Self {
            mode: DetailMode::Edit,
            subject: todo.subject.clone(),
            description: todo.description.clone(),
            created_at: Some(todo.created_at),
            closed_at: todo.closed_at,
            last_modified_at: Some(todo.last_modified_at),
            current_field: 0,
        }
    }

    pub fn new_for_creation() -> Self {
        Self {
            mode: DetailMode::New,
            subject: String::new(),
            description: String::new(),
            created_at: None,
            closed_at: None,
            last_modified_at: None,
            current_field: 0,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        // Create a centered popup
        let popup_area = centered_rect(80, 70, area);
        
        // Clear the background
        frame.render_widget(Clear, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Subject
                Constraint::Min(8),     // Description
                Constraint::Length(6),  // Metadata
                Constraint::Length(3),  // Controls
            ])
            .split(popup_area);

        let title = match self.mode {
            DetailMode::View => "Todo Details",
            DetailMode::Edit => "Edit Todo",
            DetailMode::New => "New Todo",
        };

        // Subject field
        let subject_style = if self.current_field == 0 && !matches!(self.mode, DetailMode::View) {
            TokyoNightTheme::selected()
        } else {
            TokyoNightTheme::default()
        };

        let subject = Paragraph::new(self.subject.as_str())
            .style(subject_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title("Subject")
                    .title_style(TokyoNightTheme::accent()),
            );
        frame.render_widget(subject, chunks[0]);

        // Description field
        let description_style = if self.current_field == 1 && !matches!(self.mode, DetailMode::View) {
            TokyoNightTheme::selected()
        } else {
            TokyoNightTheme::default()
        };

        let description = Paragraph::new(self.description.as_str())
            .style(description_style)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title("Description")
                    .title_style(TokyoNightTheme::accent()),
            );
        frame.render_widget(description, chunks[1]);

        // Metadata
        let mut metadata_lines = vec![];
        
        if let Some(created) = self.created_at {
            metadata_lines.push(Line::from(vec![
                Span::styled("Created: ", TokyoNightTheme::accent()),
                Span::styled(created.format("%Y-%m-%d %H:%M:%S").to_string(), TokyoNightTheme::default()),
            ]));
        }

        if let Some(modified) = self.last_modified_at {
            metadata_lines.push(Line::from(vec![
                Span::styled("Modified: ", TokyoNightTheme::accent()),
                Span::styled(modified.format("%Y-%m-%d %H:%M:%S").to_string(), TokyoNightTheme::default()),
            ]));
        }

        let status = if self.closed_at.is_some() {
            ("Completed", TokyoNightTheme::completed())
        } else {
            ("Active", TokyoNightTheme::success())
        };

        metadata_lines.push(Line::from(vec![
            Span::styled("Status: ", TokyoNightTheme::accent()),
            Span::styled(status.0, status.1),
        ]));

        if let Some(closed) = self.closed_at {
            metadata_lines.push(Line::from(vec![
                Span::styled("Closed: ", TokyoNightTheme::accent()),
                Span::styled(closed.format("%Y-%m-%d %H:%M:%S").to_string(), TokyoNightTheme::completed()),
            ]));
        }

        let metadata = Paragraph::new(metadata_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title("Information")
                    .title_style(TokyoNightTheme::accent()),
            );
        frame.render_widget(metadata, chunks[2]);

        // Controls
        let controls_text = match self.mode {
            DetailMode::View => vec![
                Line::from(vec![
                    Span::styled("Controls: ", TokyoNightTheme::accent()),
                    Span::styled("e", TokyoNightTheme::active()),
                    Span::styled("=Edit  ", TokyoNightTheme::default()),
                    Span::styled("Esc", TokyoNightTheme::warning()),
                    Span::styled("=Back", TokyoNightTheme::default()),
                ]),
            ],
            DetailMode::Edit | DetailMode::New => vec![
                Line::from(vec![
                    Span::styled("Controls: ", TokyoNightTheme::accent()),
                    Span::styled("Tab", TokyoNightTheme::active()),
                    Span::styled("=Switch Field  ", TokyoNightTheme::default()),
                    Span::styled("Ctrl+S", TokyoNightTheme::success()),
                    Span::styled("=Save  ", TokyoNightTheme::default()),
                    Span::styled("Esc", TokyoNightTheme::warning()),
                    Span::styled("=Cancel", TokyoNightTheme::default()),
                ]),
            ],
        };

        let controls = Paragraph::new(controls_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title(title)
                    .title_style(TokyoNightTheme::accent()),
            );
        frame.render_widget(controls, chunks[3]);
    }

    pub fn next_field(&mut self) {
        self.current_field = (self.current_field + 1) % 2;
    }

    pub fn previous_field(&mut self) {
        self.current_field = if self.current_field == 0 { 1 } else { 0 };
    }

    pub fn add_char(&mut self, c: char) {
        match self.current_field {
            0 => self.subject.push(c),
            1 => self.description.push(c),
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        match self.current_field {
            0 => { self.subject.pop(); },
            1 => { self.description.pop(); },
            _ => {}
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.subject.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_todo() -> Todo {
        Todo::new("Test Subject".to_string(), "Test Description".to_string())
    }

    #[test]
    fn test_detail_view_creation_for_viewing() {
        let todo = create_test_todo();
        let detail_view = DetailView::new_for_viewing(&todo);
        
        assert!(matches!(detail_view.mode, DetailMode::View));
        assert_eq!(detail_view.subject, "Test Subject");
        assert_eq!(detail_view.description, "Test Description");
        assert_eq!(detail_view.current_field, 0);
        assert!(detail_view.created_at.is_some());
        assert!(detail_view.last_modified_at.is_some());
        assert_eq!(detail_view.closed_at, None);
    }

    #[test]
    fn test_detail_view_creation_for_editing() {
        let todo = create_test_todo();
        let detail_view = DetailView::new_for_editing(&todo);
        
        assert!(matches!(detail_view.mode, DetailMode::Edit));
        assert_eq!(detail_view.subject, "Test Subject");
        assert_eq!(detail_view.description, "Test Description");
        assert_eq!(detail_view.current_field, 0);
        assert!(detail_view.created_at.is_some());
        assert!(detail_view.last_modified_at.is_some());
    }

    #[test]
    fn test_detail_view_creation_for_new() {
        let detail_view = DetailView::new_for_creation();
        
        assert!(matches!(detail_view.mode, DetailMode::New));
        assert!(detail_view.subject.is_empty());
        assert!(detail_view.description.is_empty());
        assert_eq!(detail_view.current_field, 0);
        assert!(detail_view.created_at.is_none());
        assert!(detail_view.last_modified_at.is_none());
        assert_eq!(detail_view.closed_at, None);
    }

    #[test]
    fn test_field_navigation() {
        let mut detail_view = DetailView::new_for_creation();
        
        // Start at field 0
        assert_eq!(detail_view.current_field, 0);
        
        // Move to next field
        detail_view.next_field();
        assert_eq!(detail_view.current_field, 1);
        
        // Wrap around to field 0
        detail_view.next_field();
        assert_eq!(detail_view.current_field, 0);
        
        // Move to previous field (should wrap to field 1)
        detail_view.previous_field();
        assert_eq!(detail_view.current_field, 1);
        
        // Move to previous field
        detail_view.previous_field();
        assert_eq!(detail_view.current_field, 0);
    }

    #[test]
    fn test_add_char() {
        let mut detail_view = DetailView::new_for_creation();
        
        // Add to subject (field 0)
        detail_view.current_field = 0;
        detail_view.add_char('H');
        detail_view.add_char('i');
        assert_eq!(detail_view.subject, "Hi");
        
        // Add to description (field 1)
        detail_view.current_field = 1;
        detail_view.add_char('T');
        detail_view.add_char('e');
        detail_view.add_char('s');
        detail_view.add_char('t');
        assert_eq!(detail_view.description, "Test");
        
        // Subject should remain unchanged
        assert_eq!(detail_view.subject, "Hi");
    }

    #[test]
    fn test_delete_char() {
        let mut detail_view = DetailView::new_for_creation();
        
        // Set up some content
        detail_view.subject = "Hello".to_string();
        detail_view.description = "World".to_string();
        
        // Delete from subject (field 0)
        detail_view.current_field = 0;
        detail_view.delete_char();
        assert_eq!(detail_view.subject, "Hell");
        
        // Delete from description (field 1)
        detail_view.current_field = 1;
        detail_view.delete_char();
        assert_eq!(detail_view.description, "Worl");
        
        // Delete from empty field
        detail_view.subject = String::new();
        detail_view.current_field = 0;
        detail_view.delete_char();
        assert_eq!(detail_view.subject, "");
    }

    #[test]
    fn test_is_valid() {
        let mut detail_view = DetailView::new_for_creation();
        
        // Empty subject should be invalid
        assert!(!detail_view.is_valid());
        
        // Whitespace-only subject should be invalid
        detail_view.subject = "   ".to_string();
        assert!(!detail_view.is_valid());
        
        // Non-empty subject should be valid
        detail_view.subject = "Valid Subject".to_string();
        assert!(detail_view.is_valid());
        
        // Subject with leading/trailing whitespace should be valid
        detail_view.subject = "  Valid Subject  ".to_string();
        assert!(detail_view.is_valid());
    }

    #[test]
    fn test_completed_todo_detail_view() {
        let mut todo = create_test_todo();
        todo.toggle_completion();
        
        let detail_view = DetailView::new_for_viewing(&todo);
        
        assert!(detail_view.closed_at.is_some());
        assert_eq!(detail_view.closed_at, todo.closed_at);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}