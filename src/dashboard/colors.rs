//! Color Scheme Definitions
//!
//! Defines color schemes for the TUI dashboard.

use ratatui::style::{Color, Modifier, Style};

/// Color scheme for dashboard
#[derive(Debug, Clone)]
pub struct ColorScheme {
    /// Running state color
    pub running: Color,
    /// Idle state color
    pub idle: Color,
    /// Error state color
    pub error: Color,
    /// Positive PnL color
    pub positive_pnl: Color,
    /// Negative PnL color
    pub negative_pnl: Color,
    /// Border color
    pub border: Color,
    /// Header color
    pub header: Color,
    /// Text color
    pub text: Color,
    /// Dim text color
    pub text_dim: Color,
    /// Highlight color
    pub highlight: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self::default_dark()
    }
}

impl ColorScheme {
    /// Create default dark theme
    pub fn default_dark() -> Self {
        Self {
            running: Color::Green,
            idle: Color::Yellow,
            error: Color::Red,
            positive_pnl: Color::Green,
            negative_pnl: Color::Red,
            border: Color::Blue,
            header: Color::Cyan,
            text: Color::Gray,
            text_dim: Color::DarkGray,
            highlight: Color::White,
        }
    }

    /// Create light theme
    pub fn light() -> Self {
        Self {
            running: Color::DarkGreen,
            idle: Color::DarkYellow,
            error: Color::Red,
            positive_pnl: Color::DarkGreen,
            negative_pnl: Color::Red,
            border: Color::Blue,
            header: Color::Cyan,
            text: Color::Black,
            text_dim: Color::DarkGray,
            highlight: Color::White,
        }
    }

    /// Create mono theme (for terminals without color support)
    pub fn mono() -> Self {
        Self {
            running: Color::White,
            idle: Color::White,
            error: Color::White,
            positive_pnl: Color::White,
            negative_pnl: Color::White,
            border: Color::White,
            header: Color::White,
            text: Color::White,
            text_dim: Color::Gray,
            highlight: Color::White,
        }
    }

    /// Get style for running state
    pub fn running_style(&self) -> Style {
        Style::default()
            .fg(self.running)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for idle state
    pub fn idle_style(&self) -> Style {
        Style::default().fg(self.idle)
    }

    /// Get style for error state
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).add_modifier(Modifier::BOLD)
    }

    /// Get style for positive PnL
    pub fn positive_pnl_style(&self) -> Style {
        Style::default()
            .fg(self.positive_pnl)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for negative PnL
    pub fn negative_pnl_style(&self) -> Style {
        Style::default()
            .fg(self.negative_pnl)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for border
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Get style for header
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.header)
            .add_modifier(Modifier::BOLD)
    }

    /// Get style for normal text
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    /// Get style for dim text
    pub fn text_dim_style(&self) -> Style {
        Style::default().fg(self.text_dim)
    }

    /// Get style for highlight
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_color_scheme() {
        let scheme = ColorScheme::default();
        assert_eq!(scheme.running, Color::Green);
        assert_eq!(scheme.error, Color::Red);
    }

    #[test]
    fn test_mono_scheme() {
        let scheme = ColorScheme::mono();
        assert_eq!(scheme.running, Color::White);
        assert_eq!(scheme.error, Color::White);
    }
}
