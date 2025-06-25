use crate::ui::theme::TokyoNightTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct ConfirmDialog {
    pub message: String,
    pub title: String,
}

impl ConfirmDialog {
    pub fn new(title: String, message: String) -> Self {
        Self { title, message }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let popup_area = centered_rect(50, 30, area);
        
        // Clear the background
        frame.render_widget(Clear, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),     // Message
                Constraint::Length(3),  // Controls
            ])
            .split(popup_area);

        // Message
        let message_lines = vec![
            Line::from(Span::styled(&self.message, TokyoNightTheme::default())),
            Line::from(""),
            Line::from(Span::styled("Are you sure?", TokyoNightTheme::warning().add_modifier(Modifier::BOLD))),
        ];

        let message = Paragraph::new(message_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title(self.title.as_str())
                    .title_style(TokyoNightTheme::error().add_modifier(Modifier::BOLD)),
            );
        frame.render_widget(message, chunks[0]);

        // Controls
        let controls_text = vec![
            Line::from(vec![
                Span::styled("⚠️  ", TokyoNightTheme::warning()),
                Span::styled("y", TokyoNightTheme::error()),
                Span::styled("=Yes  ", TokyoNightTheme::default()),
                Span::styled("n", TokyoNightTheme::success()),
                Span::styled("/", TokyoNightTheme::default()),
                Span::styled("Esc", TokyoNightTheme::success()),
                Span::styled("=No", TokyoNightTheme::default()),
            ]),
        ];

        let controls = Paragraph::new(controls_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border()),
            );
        frame.render_widget(controls, chunks[1]);
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