use crate::data::Todo;
use crate::ui::theme::TokyoNightTheme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, Cell, TableState},
    Frame,
};

pub struct MainView {
    pub table_state: TableState,
}

impl MainView {
    pub fn new() -> Self {
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        
        Self {
            table_state,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, todos: &[&Todo]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Todo list
                Constraint::Length(3),  // Footer
            ])
            .split(area);

        // Header
        let header = Paragraph::new("📝 TodoCLI - Terminal Todo Manager")
            .style(TokyoNightTheme::accent().add_modifier(Modifier::BOLD))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border())
                    .title("TodoCLI")
                    .title_style(TokyoNightTheme::accent()),
            );
        frame.render_widget(header, chunks[0]);

        // Todo table with columns
        let rows: Vec<Row> = todos
            .iter()
            .enumerate()
            .map(|(_i, todo)| {
                let style = if todo.is_completed() {
                    TokyoNightTheme::completed()
                } else {
                    TokyoNightTheme::default()
                };

                let status_icon = if todo.is_completed() {
                    "🔴"
                } else {
                    todo.status_icon()
                };

                let subject = &todo.subject;
                let last_modified = todo.last_modified_at.format("%Y-%m-%d %H:%M").to_string();

                Row::new(vec![
                    Cell::from(status_icon).style(style),
                    Cell::from(subject.as_str()).style(style),
                    Cell::from(last_modified).style(style),
                ])
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(3),      // Status icon column
                Constraint::Min(20),        // Subject column (flexible)
                Constraint::Length(16),     // Last modified column
            ]
        )
        .header(
            Row::new(vec![
                Cell::from("📋"),
                Cell::from("Subject"),
                Cell::from("Last Modified"),
            ])
            .style(TokyoNightTheme::accent().add_modifier(Modifier::BOLD))
            .bottom_margin(1)
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(TokyoNightTheme::border())
                .title("📝 All Todos")
                .title_style(TokyoNightTheme::accent()),
        )
        .highlight_style(TokyoNightTheme::selected())
        .highlight_symbol("▶ ");

        frame.render_stateful_widget(table, chunks[1], &mut self.table_state);

        // Footer with controls
        let footer_text = vec![
            Line::from(vec![
                Span::styled("💡 Controls: ", TokyoNightTheme::accent()),
                Span::styled("Enter", TokyoNightTheme::active()),
                Span::styled("=View/Edit  ", TokyoNightTheme::default()),
                Span::styled("d", TokyoNightTheme::active()),
                Span::styled("=Toggle  ", TokyoNightTheme::default()),
                Span::styled("n", TokyoNightTheme::active()),
                Span::styled("=New  ", TokyoNightTheme::default()),
                Span::styled("x", TokyoNightTheme::error()),
                Span::styled("=Delete  ", TokyoNightTheme::default()),
                Span::styled("q", TokyoNightTheme::warning()),
                Span::styled("=Quit", TokyoNightTheme::default()),
            ]),
        ];

        let footer = Paragraph::new(footer_text)
            .style(TokyoNightTheme::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(TokyoNightTheme::border()),
            );
        frame.render_widget(footer, chunks[2]);
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => (i + 1) % len,
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.table_state.selected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_view_creation() {
        let main_view = MainView::new();
        assert_eq!(main_view.selected_index(), Some(0));
    }

    #[test]
    fn test_navigation_next() {
        let mut main_view = MainView::new();
        
        // Test with empty list
        main_view.next(0);
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test with single item
        main_view.next(1);
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test with multiple items
        main_view.next(3);
        assert_eq!(main_view.selected_index(), Some(1));
        
        main_view.next(3);
        assert_eq!(main_view.selected_index(), Some(2));
        
        // Test wrapping around
        main_view.next(3);
        assert_eq!(main_view.selected_index(), Some(0));
    }

    #[test]
    fn test_navigation_previous() {
        let mut main_view = MainView::new();
        
        // Test with empty list
        main_view.previous(0);
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test with single item
        main_view.previous(1);
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test with multiple items - should wrap to end
        main_view.previous(3);
        assert_eq!(main_view.selected_index(), Some(2));
        
        main_view.previous(3);
        assert_eq!(main_view.selected_index(), Some(1));
        
        main_view.previous(3);
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test wrapping around to end again
        main_view.previous(3);
        assert_eq!(main_view.selected_index(), Some(2));
    }

    #[test]
    fn test_navigation_with_no_selection() {
        let mut main_view = MainView::new();
        main_view.table_state.select(None);
        
        main_view.next(3);
        assert_eq!(main_view.selected_index(), Some(0));
        
        main_view.table_state.select(None);
        main_view.previous(3);
        assert_eq!(main_view.selected_index(), Some(0));
    }

    #[test]
    fn test_selection_state() {
        let mut main_view = MainView::new();
        
        // Test initial selection
        assert_eq!(main_view.selected_index(), Some(0));
        
        // Test manual selection
        main_view.table_state.select(Some(5));
        assert_eq!(main_view.selected_index(), Some(5));
        
        // Test no selection
        main_view.table_state.select(None);
        assert_eq!(main_view.selected_index(), None);
    }
}