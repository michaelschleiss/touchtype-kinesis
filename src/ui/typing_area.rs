use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::engine::TypingSession;

/// Renders the typing exercise text with color-coded feedback.
pub struct TypingAreaWidget<'a> {
    session: &'a TypingSession,
}

impl<'a> TypingAreaWidget<'a> {
    pub fn new(session: &'a TypingSession) -> Self {
        Self { session }
    }
}

impl Widget for TypingAreaWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans: Vec<Span> = Vec::new();

        for (i, &ch) in self.session.text.iter().enumerate() {
            let display_char = if ch == ' ' { ' ' } else { ch };

            let style = if i < self.session.cursor {
                if self.session.errors[i] {
                    // Error: red background, universally visible
                    Style::default().fg(Color::White).bg(Color::Red)
                } else {
                    // Correct: dim, recedes
                    Style::default().fg(Color::DarkGray)
                }
            } else if i == self.session.cursor {
                // Cursor: high-contrast, unmissable
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                // Upcoming: bright, readable
                Style::default().fg(Color::White)
            };

            spans.push(Span::styled(display_char.to_string(), style));
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).wrap(Wrap { trim: false });
        paragraph.render(area, buf);
    }
}

/// Build a stats line showing live WPM and accuracy.
pub fn stats_line(session: &TypingSession) -> Line<'static> {
    let elapsed = session.elapsed_secs();
    let acc = session.accuracy() * 100.0;
    let errors = session.errors.iter().take(session.cursor).filter(|&&e| e).count();
    let total = session.text.len();
    let typed = session.cursor;

    let mins = (elapsed as u32) / 60;
    let secs = (elapsed as u32) % 60;

    // Suppress WPM until meaningful (10+ chars and 2+ seconds)
    let wpm_display = if typed >= 10 && elapsed >= 2.0 {
        format!("{:.0}", session.net_wpm())
    } else {
        "--".to_string()
    };

    let wpm_val: f64 = if typed >= 10 && elapsed >= 2.0 {
        session.net_wpm()
    } else {
        0.0
    };

    let wpm_color = if wpm_val >= 60.0 {
        Color::Green
    } else if wpm_val >= 30.0 {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let acc_color = if acc >= 95.0 {
        Color::Green
    } else if acc >= 85.0 {
        Color::Yellow
    } else if typed > 0 {
        Color::Red
    } else {
        Color::DarkGray
    };

    let err_color = if errors == 0 {
        Color::DarkGray
    } else {
        Color::Red
    };

    Line::from(vec![
        Span::styled("WPM ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            wpm_display,
            Style::default().fg(wpm_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  Acc ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.0}%", acc),
            Style::default().fg(acc_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  Err ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", errors),
            Style::default().fg(err_color),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{}:{:02}", mins, secs),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{}/{}", typed, total),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}
