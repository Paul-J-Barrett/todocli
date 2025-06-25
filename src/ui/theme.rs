use ratatui::style::{Color, Style};

pub struct TokyoNightTheme;

impl TokyoNightTheme {
    pub const BACKGROUND: Color = Color::Rgb(26, 27, 38);      // #1a1b26
    pub const FOREGROUND: Color = Color::Rgb(192, 202, 245);   // #c0caf5
    pub const ACTIVE: Color = Color::Rgb(122, 162, 247);       // #7aa2f7
    pub const COMPLETED: Color = Color::Rgb(247, 118, 142);    // #f7768e (red)
    pub const BORDER: Color = Color::Rgb(65, 72, 104);         // #414868
    pub const ACCENT: Color = Color::Rgb(187, 154, 247);       // #bb9af7
    pub const SUCCESS: Color = Color::Rgb(158, 206, 106);      // #9ece6a
    pub const WARNING: Color = Color::Rgb(255, 158, 100);      // #ff9e64
    pub const ERROR: Color = Color::Rgb(247, 118, 142);        // #f7768e
    // pub const COMMENT: Color = Color::Rgb(86, 95, 137);        // #565f89

    pub fn default() -> Style {
        Style::default()
            .fg(Self::FOREGROUND)
            .bg(Self::BACKGROUND)
    }

    pub fn active() -> Style {
        Style::default()
            .fg(Self::ACTIVE)
            .bg(Self::BACKGROUND)
    }

    pub fn completed() -> Style {
        Style::default()
            .fg(Self::COMPLETED)
            .bg(Self::BACKGROUND)
    }

    pub fn border() -> Style {
        Style::default()
            .fg(Self::BORDER)
    }

    pub fn accent() -> Style {
        Style::default()
            .fg(Self::ACCENT)
            .bg(Self::BACKGROUND)
    }

    pub fn success() -> Style {
        Style::default()
            .fg(Self::SUCCESS)
            .bg(Self::BACKGROUND)
    }

    pub fn warning() -> Style {
        Style::default()
            .fg(Self::WARNING)
            .bg(Self::BACKGROUND)
    }

    pub fn error() -> Style {
        Style::default()
            .fg(Self::ERROR)
            .bg(Self::BACKGROUND)
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(Self::BACKGROUND)
            .bg(Self::ACTIVE)
    }
}