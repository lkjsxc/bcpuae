//! Drawing functions for UI components

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use crate::operations::commands::EditorMode;
use crate::system::notifications::{Notification, NotifyLevel};

use crate::ui::state::EditorState;
use crate::ui::theme::Theme;

/// Draw the editor area with line numbers and content
pub fn draw_editor(frame: &mut Frame, state: &EditorState, area: Rect, theme: &Theme) {
    let start_line = state.start_line();
    let viewport_height = area.height as usize;
    let end_line = (start_line + viewport_height).min(state.document.buffer().line_count());

    let mut lines: Vec<Line> = Vec::new();
    let sparkle = state.idle_tracker.should_sparkle();

    for (line_idx, i) in (start_line..end_line).enumerate() {
        let line_num = format!("{:>4} │ ", i + 1);
        let line_content = state.document.buffer().get_line(i).unwrap_or("");

        // Line number is always gray
        let num_span = Span::styled(line_num, Style::default().fg(theme.line_number_fg));

        // Text content with optional sparkle effect
        let content_span = if sparkle {
            // Create dazzling rainbow effect with per-character independent cycling
            let spans: Vec<Span> = line_content
                .chars()
                .enumerate()
                .map(|(char_idx, ch)| {
                    // Each character has its own independent color cycle
                    let color = state.idle_tracker.char_sparkle_color(line_idx, char_idx);
                    Span::styled(ch.to_string(), Style::default().fg(color))
                })
                .collect();

            // Combine all character spans - we return a line with multiple spans
            lines.push(Line::from(
                std::iter::once(num_span)
                    .chain(spans)
                    .collect::<Vec<_>>(),
            ));
            continue;
        } else {
            Span::raw(line_content)
        };

        lines.push(Line::from(vec![num_span, content_span]));
    }

    // Fill remaining viewport with empty lines
    for _ in lines.len()..viewport_height {
        lines.push(Line::from("~"));
    }

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, area);
}

/// Draw notification area
pub fn draw_notifications(
    frame: &mut Frame,
    notifications: &[Notification],
    area: Rect,
    theme: &Theme,
) {
    let mut lines: Vec<Line> = Vec::new();

    // Show up to 3 most recent notifications
    let start = notifications.len().saturating_sub(3);
    for notif in &notifications[start..] {
        let (prefix, color) = match notif.level {
            NotifyLevel::Info => ("[INFO]", theme.notify_color(&NotifyLevel::Info)),
            NotifyLevel::Success => ("[OK]  ", theme.notify_color(&NotifyLevel::Success)),
            NotifyLevel::Warning => ("[WARN]", theme.notify_color(&NotifyLevel::Warning)),
            NotifyLevel::Fatal => ("[ERR] ", theme.notify_color(&NotifyLevel::Fatal)),
        };

        let spans = vec![
            Span::styled(
                prefix,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::raw(notif.message.clone()),
        ];

        lines.push(Line::from(spans));
    }

    // Fill remaining rows
    while lines.len() < 3 {
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, area);
}

/// Draw status bar with mode, filename, and dirty indicator
pub fn draw_status_bar(frame: &mut Frame, state: &EditorState, area: Rect, theme: &Theme) {
    let mode_str = state.mode.to_string();
    let mode_color = theme.mode_color(&mode_str);

    let mut spans = vec![
        Span::styled(
            format!(" {} ", mode_str),
            Style::default()
                .bg(mode_color)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::raw(state.document.display_name()),
    ];

    if state.document.is_dirty() {
        spans.push(Span::styled(" [+]", Style::default().fg(Color::Yellow)));
    }

    // Add command buffer if in command mode
    if let EditorMode::Command { buffer } = &state.mode {
        spans.push(Span::raw(format!(" :{}", buffer)));
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(Text::from(vec![line]));
    frame.render_widget(paragraph, area);
}
