//! Custom Widgets
//!
//! Custom ratatui widgets for the dashboard.

use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::colors::ColorScheme;
use crate::dashboard::{AgentState, AgentStatus, BalanceData, CoinPrice, Notification, PositionData};

/// Render status panel
pub fn render_status_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    states: &[AgentState],
    colors: &ColorScheme,
) {
    let block = Block::default()
        .title(" Status ")
        .borders(Borders::ALL)
        .border_style(colors.border_style());

    // Build status lines
    let mut lines = vec![
        Line::from(vec![
            Span::styled("Agent", Style::default().fg(colors.header)),
            Span::raw("  "),
            Span::styled("State", Style::default().fg(colors.header)),
            Span::raw("   "),
            Span::styled("Last Update", Style::default().fg(colors.header)),
        ]),
        Line::from("─".repeat(area.width as usize).to_string()),
    ];

    for state in states {
        let status_style = match state.status {
            AgentStatus::Running => colors.running_style(),
            AgentStatus::Idle => colors.idle_style(),
            AgentStatus::Error => colors.error_style(),
            AgentStatus::Disconnected => colors.error_style(),
        };

        let time_ago = format_time_ago(state.last_update);

        lines.push(Line::from(vec![
            Span::raw(&state.name),
            Span::raw("  "),
            Span::styled(state.status.as_str(), status_style),
            Span::raw("   "),
            Span::styled(time_ago, colors.text_dim_style()),
        ]));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Render position panel
pub fn render_position_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    position: Option<&PositionData>,
    colors: &ColorScheme,
) {
    let block = Block::default()
        .title(" Position ")
        .borders(Borders::ALL)
        .border_style(colors.border_style());

    let text = if let Some(pos) = position {
        let pnl_style = if pos.pnl_rate >= 0.0 {
            colors.positive_pnl_style()
        } else {
            colors.negative_pnl_style()
        };

        let pnl_sign = if pos.pnl_rate >= 0.0 { "+" } else { "" };

        vec![
            Line::from(vec![
                Span::styled("Market: ", colors.text_style()),
                Span::styled(&pos.market, colors.highlight_style()),
            ]),
            Line::from(vec![
                Span::styled("Entry:  ", colors.text_style()),
                Span::styled(format!("{:,.0}", pos.entry_price), colors.text_style()),
            ]),
            Line::from(vec![
                Span::styled("Current:", colors.text_style()),
                Span::styled(format!("{:,.0}", pos.current_price), colors.text_style()),
            ]),
            Line::from(vec![
                Span::styled("Amount: ", colors.text_style()),
                Span::styled(format!("{:.6}", pos.amount), colors.text_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("PnL: ", colors.text_style()),
                Span::styled(
                    format!(
                        "{}{:.2}% ({}{:.0} KRW)",
                        pnl_sign,
                        pos.pnl_rate * 100.0,
                        pnl_sign,
                        pos.pnl_amount
                    ),
                    pnl_style,
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Target:  ", colors.text_style()),
                Span::styled(
                    format!(
                        "+{:.0}% (Stop)",
                        pos.take_profit / pos.entry_price * 100.0 - 100.0
                    ),
                    colors.positive_pnl_style(),
                ),
            ]),
            Line::from(vec![
                Span::styled("StopLoss:", colors.text_style()),
                Span::styled(
                    format!(
                        "-{:.0}% (Safe)",
                        100.0 - pos.stop_loss / pos.entry_price * 100.0
                    ),
                    colors.negative_pnl_style(),
                ),
            ]),
        ]
    } else {
        vec![Line::from(vec![Span::styled(
            "No Position",
            colors.text_dim_style(),
        )])]
    };

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Render balance panel
pub fn render_balance_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    balance: &BalanceData,
    colors: &ColorScheme,
) {
    let block = Block::default()
        .title(" Balance ")
        .borders(Borders::ALL)
        .border_style(colors.border_style());

    let pnl_style = if balance.today_pnl_rate >= 0.0 {
        colors.positive_pnl_style()
    } else {
        colors.negative_pnl_style()
    };

    let pnl_sign = if balance.today_pnl_rate >= 0.0 {
        "+"
    } else {
        ""
    };

    let text = vec![
        Line::from(vec![
            Span::styled("KRW Balance: ", colors.text_style()),
            Span::styled(
                format!("{:,.0}", balance.krw_balance),
                colors.highlight_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Available:  ", colors.text_style()),
            Span::styled(
                format!("{:,.0}", balance.available_krw),
                colors.text_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("Locked:     ", colors.text_style()),
            Span::styled(format!("{:,.0}", balance.locked_krw), colors.text_style()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Total Asset: ", colors.text_style()),
            Span::styled(
                format!("{:,.0}", balance.total_asset_value),
                colors.highlight_style(),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Today's PnL: ", colors.text_style()),
            Span::styled(
                format!(
                    "{}{:.0} ({}{:.2}%)",
                    pnl_sign,
                    balance.today_pnl,
                    pnl_sign,
                    balance.today_pnl_rate * 100.0
                ),
                pnl_style,
            ),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Render market panel
pub fn render_market_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    prices: &[CoinPrice],
    colors: &ColorScheme,
) {
    let block = Block::default()
        .title(" Market (Top 20) ")
        .borders(Borders::ALL)
        .border_style(colors.border_style());

    let items: Vec<ListItem> = prices
        .iter()
        .take(20)
        .map(|price| {
            let change_style = if price.change_rate >= 0.0 {
                colors.positive_pnl_style()
            } else {
                colors.negative_pnl_style()
            };

            let change_sign = if price.change_rate >= 0.0 { "+" } else { "" };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<12}", price.market), colors.text_style()),
                Span::styled(format!("{:>12,.0}", price.price), colors.text_style()),
                Span::raw(" "),
                Span::styled(
                    format!("{}{:.2}%", change_sign, price.change_rate * 100.0),
                    change_style,
                ),
            ]))
        })
        .collect();

    let list = List::new(items).block(block).alignment(Alignment::Left);

    frame.render_widget(list, area);
}

/// Render notifications panel
pub fn render_notifications_panel(
    frame: &mut Frame,
    area: ratatui::layout::Rect,
    notifications: &[Notification],
    colors: &ColorScheme,
) {
    let block = Block::default()
        .title(" Notifications ")
        .borders(Borders::ALL)
        .border_style(colors.border_style());

    // Show latest notifications (up to area height - 2 for borders)
    let max_lines = area.height.saturating_sub(2) as usize;
    let items: Vec<ListItem> = notifications
        .iter()
        .rev()
        .take(max_lines)
        .map(|notif| {
            ListItem::new(Line::from(vec![Span::styled(
                notif.format(),
                colors.text_style(),
            )]))
        })
        .collect();

    let list = List::new(items).block(block).alignment(Alignment::Left);

    frame.render_widget(list, area);
}

/// Render help footer
pub fn render_help_footer(frame: &mut Frame, area: ratatui::layout::Rect, colors: &ColorScheme) {
    let text = vec![Line::from(vec![
        Span::styled("[", colors.text_dim_style()),
        Span::styled("q", colors.highlight_style()),
        Span::styled("] Quit ", colors.text_dim_style()),
        Span::styled("[", colors.text_dim_style()),
        Span::styled("p", colors.highlight_style()),
        Span::styled("] Pause ", colors.text_dim_style()),
        Span::styled("[", colors.text_dim_style()),
        Span::styled("r", colors.highlight_style()),
        Span::styled("] Resume ", colors.text_dim_style()),
        Span::styled("[", colors.text_dim_style()),
        Span::styled("?", colors.highlight_style()),
        Span::styled("] Help", colors.text_dim_style()),
    ])];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Format time ago for display
fn format_time_ago(dt: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_seconds() < 60 {
        format!("{}s ago", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else {
        format!("{}d ago", duration.num_days())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_format_time_ago() {
        let now = Utc::now();
        assert!(format_time_ago(now).ends_with("s ago"));

        let one_min_ago = now - chrono::Duration::seconds(60);
        assert!(format_time_ago(one_min_ago).ends_with("m ago"));
    }
}
