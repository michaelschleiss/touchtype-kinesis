use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::persistence::UserProgress;

pub struct ProgressWidget<'a> {
    pub progress: &'a UserProgress,
    pub total_lessons: usize,
}

impl Widget for ProgressWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(2),  // header
            Constraint::Length(7),  // stats
            Constraint::Min(4),    // per-key breakdown (flexible)
            Constraint::Length(2), // help
        ])
        .split(area);

        // Header
        let header = Paragraph::new(Line::from(Span::styled(
            " Progress",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        header.render(chunks[0], buf);

        // Summary stats
        let lessons_done = self.progress.highest_lesson;
        let total_sessions = self.progress.total_sessions;
        let best_wpm = self.progress.best_wpm;
        let best_acc = self.progress.best_accuracy * 100.0;

        let bar_width = 20usize;
        let filled =
            (lessons_done as f64 / self.total_lessons as f64 * bar_width as f64) as usize;
        let bar: String = format!(
            "[{}{}] {}/{}",
            "#".repeat(filled),
            "-".repeat(bar_width - filled),
            lessons_done,
            self.total_lessons
        );

        let stats = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("    Lessons:   ", Style::default().fg(Color::DarkGray)),
                Span::styled(bar, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("    Sessions:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}", total_sessions),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("    Best WPM:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{:.0}", best_wpm),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("    Best Acc:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{:.1}%", best_acc),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ]);
        stats.render(chunks[1], buf);

        // Per-key accuracy (flexible height)
        let mut key_stats: Vec<_> = self.progress.per_key_accuracy.iter().collect();
        key_stats.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        let max_keys = (chunks[2].height as usize).saturating_sub(2);
        let mut key_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "    Keys to Improve:",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        if key_stats.is_empty() {
            key_lines.push(Line::from(Span::styled(
                "    No data yet",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (ch, acc) in key_stats.iter().take(max_keys.min(8)) {
                let display = if **ch == ' ' {
                    "Space".to_string()
                } else {
                    ch.to_string()
                };
                let pct = *acc * 100.0;
                key_lines.push(Line::from(vec![
                    Span::styled("      ", Style::default()),
                    Span::styled(
                        format!("{:<6}", display),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("{:.0}%", pct),
                        Style::default().fg(if pct >= 95.0 {
                            Color::Green
                        } else if pct >= 85.0 {
                            Color::Yellow
                        } else {
                            Color::Red
                        }),
                    ),
                ]));
            }
        }
        let key_widget = Paragraph::new(key_lines);
        key_widget.render(chunks[2], buf);

        // Help
        let help = Paragraph::new(Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::White)),
            Span::styled("] Back to Menu", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center);
        help.render(chunks[3], buf);
    }
}
