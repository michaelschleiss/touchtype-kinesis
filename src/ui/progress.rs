use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::engine::curriculum::ALL_STAGES;
use crate::engine::Curriculum;
use crate::persistence::UserProgress;

pub struct ProgressWidget<'a> {
    pub progress: &'a UserProgress,
    pub curriculum: &'a Curriculum,
}

impl Widget for ProgressWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(2),  // header
            Constraint::Length(5),  // summary stats
            Constraint::Min(4),    // stage breakdown + keys (flexible)
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
        let total_lessons = self.curriculum.lessons.len();
        let lessons_done = self.progress.highest_lesson.min(total_lessons);
        let total_sessions = self.progress.total_sessions;
        let best_wpm = self.progress.best_wpm;
        let best_acc = self.progress.best_accuracy * 100.0;

        let bar_width = 20usize;
        let filled = (lessons_done as f64 / total_lessons as f64 * bar_width as f64) as usize;
        let bar: String = format!(
            "[{}{}] {}/{}",
            "#".repeat(filled),
            "-".repeat(bar_width - filled),
            lessons_done,
            total_lessons
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

        // Stage breakdown + per-key accuracy (share the flexible space)
        let detail_chunks = Layout::vertical([
            Constraint::Min(4),    // stage breakdown
            Constraint::Length(1), // spacer
            Constraint::Min(4),    // per-key
        ])
        .split(chunks[2]);

        // Stage breakdown
        let mut stage_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "    Stages:",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )),
        ];

        for stage in ALL_STAGES {
            let info = stage.info();
            let stage_lessons = self.curriculum.lessons_for_stage(*stage);
            let completed = stage_lessons
                .iter()
                .filter(|l| l.id < self.progress.highest_lesson)
                .count();
            let total = stage_lessons.len();
            let all_done = completed == total;

            let status_color = if all_done {
                Color::Green
            } else if completed > 0 {
                Color::Yellow
            } else {
                Color::DarkGray
            };

            stage_lines.push(Line::from(vec![
                Span::styled("      ", Style::default()),
                Span::styled(
                    format!("{:<22}", info.name),
                    Style::default().fg(status_color),
                ),
                Span::styled(
                    format!("{}/{}", completed, total),
                    Style::default().fg(status_color),
                ),
            ]));
        }

        let stage_widget = Paragraph::new(stage_lines);
        stage_widget.render(detail_chunks[0], buf);

        // Per-key accuracy
        let mut key_stats: Vec<_> = self.progress.per_key_accuracy.iter().collect();
        key_stats.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        let max_keys = (detail_chunks[2].height as usize).saturating_sub(2);
        let mut key_lines = vec![
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
        key_widget.render(detail_chunks[2], buf);

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
