//! Layout Management
//!
//! Manages the 4-panel layout for the dashboard.

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};

use super::ColorScheme;

/// Dashboard layout configuration
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Enable compact mode for small terminals
    pub compact_mode: bool,
    /// Show help footer
    pub show_help: bool,
    /// Terminal width
    pub terminal_width: u16,
    /// Terminal height
    pub terminal_height: u16,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            compact_mode: false,
            show_help: true,
            terminal_width: 120,
            terminal_height: 40,
        }
    }
}

impl LayoutConfig {
    /// Create from terminal size
    pub fn from_terminal_size(width: u16, height: u16) -> Self {
        let compact_mode = width < 80 || height < 24;

        Self {
            compact_mode,
            show_help: true,
            terminal_width: width,
            terminal_height: height,
        }
    }

    /// Check if compact mode is enabled
    pub fn is_compact(&self) -> bool {
        self.compact_mode
    }

    /// Get main content area (excluding header and footer)
    pub fn main_content_area(&self, area: Rect) -> Rect {
        if self.show_help {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(area);

            chunks[0]
        } else {
            area
        }
    }
}

/// Dashboard layout manager
pub struct DashboardLayout {
    config: LayoutConfig,
}

impl DashboardLayout {
    /// Create new layout manager
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }

    /// Create with default config
    pub fn default_config() -> Self {
        Self::new(LayoutConfig::default())
    }

    /// Create from terminal size
    pub fn from_terminal_size(width: u16, height: u16) -> Self {
        Self::new(LayoutConfig::from_terminal_size(width, height))
    }

    /// Calculate and return all panel areas
    pub fn calculate_panels(&self, area: Rect) -> PanelAreas {
        // Main content area (excluding header/footer)
        let main_area = self.config.main_content_area(area);

        if self.config.is_compact() {
            // Compact mode: stack panels vertically
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(25), // Status
                        Constraint::Percentage(25), // Position
                        Constraint::Percentage(25), // Balance
                        Constraint::Percentage(25), // Market
                    ]
                    .as_ref(),
                )
                .split(main_area);

            PanelAreas {
                status: chunks[0],
                position: chunks[1],
                balance: chunks[2],
                market: chunks[3],
                notifications: Rect::default(), // Hidden in compact mode
                help: if self.config.show_help {
                    Rect {
                        x: area.x,
                        y: area.height.saturating_sub(1),
                        width: area.width,
                        height: 1,
                    }
                } else {
                    Rect::default()
                },
            }
        } else {
            // Normal mode: 2x2 grid + notifications panel
            // Top half: Status | Position
            // Bottom half: Balance | Market
            // Bottom: Notifications

            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(50), // Top half
                        Constraint::Percentage(40), // Middle half
                        Constraint::Percentage(10), // Bottom notifications
                    ]
                    .as_ref(),
                )
                .split(main_area);

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(vertical_chunks[0]);

            let middle_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(vertical_chunks[1]);

            PanelAreas {
                status: top_chunks[0],
                position: top_chunks[1],
                balance: middle_chunks[0],
                market: middle_chunks[1],
                notifications: vertical_chunks[2],
                help: if self.config.show_help {
                    Rect {
                        x: area.x,
                        y: area.height.saturating_sub(1),
                        width: area.width,
                        height: 1,
                    }
                } else {
                    Rect::default()
                },
            }
        }
    }

    /// Render header block
    pub fn render_header(&self, frame: &mut Frame, area: Rect, title: &str) {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Cyan));

        frame.render_widget(block, area);
    }

    /// Render panel with title
    pub fn render_panel(
        &self,
        frame: &mut Frame,
        area: Rect,
        title: &str,
        color_scheme: &ColorScheme,
    ) {
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(color_scheme.border_style());

        frame.render_widget(block, area);
    }
}

/// Panel areas for the dashboard
#[derive(Debug, Clone)]
pub struct PanelAreas {
    /// Status panel area
    pub status: Rect,
    /// Position panel area
    pub position: Rect,
    /// Balance panel area
    pub balance: Rect,
    /// Market panel area
    pub market: Rect,
    /// Notifications panel area
    pub notifications: Rect,
    /// Help footer area
    pub help: Rect,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert!(!config.compact_mode);
        assert!(config.show_help);
    }

    #[test]
    fn test_compact_mode_detection() {
        let config = LayoutConfig::from_terminal_size(79, 24);
        assert!(config.is_compact());

        let config = LayoutConfig::from_terminal_size(80, 24);
        assert!(!config.is_compact());
    }

    #[test]
    fn test_panel_areas_creation() {
        let layout = DashboardLayout::default_config();
        let area = Rect::new(0, 0, 120, 40);
        let panels = layout.calculate_panels(area);

        // All panels should have non-zero areas
        assert!(panels.status.width > 0);
        assert!(panels.position.width > 0);
        assert!(panels.balance.width > 0);
        assert!(panels.market.width > 0);
    }
}
