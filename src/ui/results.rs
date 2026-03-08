use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::engine::{ProgressionResult, TypingSession};

pub struct ResultsWidget<'a> {
    pub session: &'a TypingSession,
    pub lesson_name: &'a str,
    pub result: Option<&'a ProgressionResult>,
}

impl Widget for ResultsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let passed = self.result.map(|r| r.passed).unwrap_or(false);

        let chunks = Layout::vertical([
            Constraint::Length(3),  // header
            Constraint::Length(9),  // main stats
            Constraint::Min(4),    // weak keys (flexible)
            Constraint::Length(2), // help
        ])
        .split(area);

        // Header
        let status_text = if passed { "PASSED!" } else { "Keep Practicing" };
        let status_color = if passed { Color::Green } else { Color::Yellow };

        let header = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                format!("  {} — {}", self.lesson_name, status_text),
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            )),
        ]);
        header.render(chunks[0], buf);

        // Main stats
        let wpm = self.session.net_wpm();
        let gross_wpm = self.session.gross_wpm();
        let acc = self.session.accuracy() * 100.0;
        let elapsed = self.session.elapsed_secs();
        let total_chars = self.session.cursor;
        let errors = self.session.errors.iter().filter(|&&e| e).count();

        let mins = (elapsed as u32) / 60;
        let secs = (elapsed as u32) % 60;

        let target_accuracy = self.result.map(|r| r.target_accuracy).unwrap_or(0.9);

        let mut stat_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("    Net WPM:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{:.0}", wpm),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  (gross: {:.0})", gross_wpm),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(vec![
                Span::styled("    Accuracy:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{:.1}%", acc),
                    Style::default()
                        .fg(if acc >= 95.0 {
                            Color::Green
                        } else if acc >= 85.0 {
                            Color::Yellow
                        } else {
                            Color::Red
                        })
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  (target: {:.0}%)", target_accuracy * 100.0),
                    Style::default().fg(Color::DarkGray),
                ),
                if let Some(result) = self.result {
                    if !result.accuracy_passed {
                        Span::styled("  MISSED", Style::default().fg(Color::Red))
                    } else {
                        Span::raw("")
                    }
                } else {
                    Span::raw("")
                },
            ]),
        ];

        // Show WPM target if this stage has one
        if let Some(result) = self.result {
            if let Some(target_wpm) = result.target_wpm {
                stat_lines.push(Line::from(vec![
                    Span::styled("    WPM Goal:  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        format!("{:.0}", target_wpm),
                        Style::default().fg(Color::DarkGray),
                    ),
                    if result.wpm_passed {
                        Span::styled("  REACHED", Style::default().fg(Color::Green))
                    } else {
                        Span::styled("  MISSED", Style::default().fg(Color::Red))
                    },
                ]));
            }
        }

        stat_lines.push(Line::from(vec![
            Span::styled("    Time:      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}:{:02}", mins, secs),
                Style::default().fg(Color::White),
            ),
        ]));
        stat_lines.push(Line::from(vec![
            Span::styled("    Chars:     ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", total_chars),
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("  ({} errors)", errors),
                Style::default().fg(if errors == 0 {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]));

        let stats = Paragraph::new(stat_lines);
        stats.render(chunks[1], buf);

        // Weak keys
        let weak = self.session.weakest_keys(5);
        let mut weak_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "    Weakest Keys:",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        if weak.is_empty() {
            weak_lines.push(Line::from(Span::styled(
                "    All keys above threshold!",
                Style::default().fg(Color::Green),
            )));
        } else {
            for (ch, acc) in &weak {
                let display = if *ch == ' ' {
                    "Space".to_string()
                } else {
                    ch.to_string()
                };
                weak_lines.push(Line::from(vec![
                    Span::styled("      ", Style::default()),
                    Span::styled(
                        format!("{:<6}", display),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:.0}%", acc * 100.0),
                        Style::default().fg(if *acc >= 0.95 {
                            Color::Green
                        } else if *acc >= 0.85 {
                            Color::Yellow
                        } else {
                            Color::Red
                        }),
                    ),
                ]));
            }
        }
        let weak_widget = Paragraph::new(weak_lines);
        weak_widget.render(chunks[2], buf);

        // Help
        let help = Paragraph::new(Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::White)),
            Span::styled("] ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                if passed { "Next Lesson" } else { "Retry" },
                Style::default().fg(Color::White),
            ),
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("r", Style::default().fg(Color::White)),
            Span::styled("] Retry  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::White)),
            Span::styled("] Menu", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center);
        help.render(chunks[3], buf);
    }
}
