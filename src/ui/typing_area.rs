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
                // Already typed
                if self.session.errors[i] {
                    // Was an error
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::CROSSED_OUT)
                } else {
                    // Correct
                    Style::default().fg(Color::DarkGray)
                }
            } else if i == self.session.cursor {
                // Current position — cursor
                Style::default()
                    .fg(Color::White)
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                // Upcoming
                Style::default().fg(Color::Gray)
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
    let wpm = session.net_wpm();
    let acc = session.accuracy() * 100.0;
    let elapsed = session.elapsed_secs();
    let progress = (session.progress() * 100.0) as u32;

    let mins = (elapsed as u32) / 60;
    let secs = (elapsed as u32) % 60;

    let wpm_color = if wpm >= 60.0 {
        Color::Green
    } else if wpm >= 30.0 {
        Color::Yellow
    } else {
        Color::White
    };

    let acc_color = if acc >= 95.0 {
        Color::Green
    } else if acc >= 85.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    Line::from(vec![
        Span::styled("  WPM: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.0}", wpm),
            Style::default().fg(wpm_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  Acc: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}%", acc),
            Style::default().fg(acc_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}:{:02}", mins, secs),
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}%", progress),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}
