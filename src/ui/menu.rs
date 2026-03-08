use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::engine::curriculum::Stage;
use crate::engine::Curriculum;

// ─── Main Menu ───

pub struct MainMenuWidget {
    pub selected: usize,
}

const MENU_ITEMS: &[(&str, &str)] = &[
    ("Learn", "Guided lessons from home row to full keyboard"),
    ("Practice", "Free typing with common words"),
    ("Test", "Timed typing test (60 seconds)"),
    ("Progress", "View your statistics and history"),
    ("Quit", "Exit KinType"),
];

impl MainMenuWidget {
    pub fn item_count() -> usize {
        MENU_ITEMS.len()
    }
}

impl Widget for MainMenuWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(5), // title
            Constraint::Min(8),   // menu items
            Constraint::Length(2), // help
        ])
        .split(area);

        // Title (centered)
        let title = vec![
            Line::from(""),
            Line::from(Span::styled(
                "K I N T Y P E",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "Touch Typing for Kinesis 360",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
        ];
        let title_widget = Paragraph::new(title).alignment(Alignment::Center);
        title_widget.render(chunks[0], buf);

        // Menu items — fixed left margin, not centered
        let left_margin = (area.width.saturating_sub(60) / 2).max(4);
        let menu_area = Rect {
            x: chunks[1].x + left_margin,
            width: chunks[1].width.saturating_sub(left_margin),
            ..chunks[1]
        };

        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(""));
        for (i, (name, desc)) in MENU_ITEMS.iter().enumerate() {
            let is_selected = i == self.selected;
            let arrow = if is_selected { "> " } else { "  " };
            let name_style = if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let desc_style = Style::default().fg(Color::DarkGray);

            lines.push(Line::from(vec![
                Span::styled(arrow, Style::default().fg(Color::Cyan)),
                Span::styled(format!("{:<12}", name), name_style),
                Span::styled(*desc, desc_style),
            ]));
        }
        let menu = Paragraph::new(lines);
        menu.render(menu_area, buf);

        // Help line
        let help = Paragraph::new(Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("j/k", Style::default().fg(Color::White)),
            Span::styled("] Navigate  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::White)),
            Span::styled("] Select  [", Style::default().fg(Color::DarkGray)),
            Span::styled("q", Style::default().fg(Color::White)),
            Span::styled("] Quit", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center);
        help.render(chunks[2], buf);
    }
}

// ─── Lesson Select ───

/// A row in the lesson select display (some are headers, some are lessons).
enum DisplayRow {
    StageHeader {
        name: &'static str,
        cognitive_goal: &'static str,
        wpm_range: (u16, u16),
        stage_complete: bool,
    },
    LessonEntry {
        lesson_id: usize,
        name: &'static str,
        description: &'static str,
        is_locked: bool,
        is_completed: bool,
    },
    Blank,
}

pub struct LessonSelectWidget<'a> {
    pub curriculum: &'a Curriculum,
    pub selected: usize,
    pub highest_unlocked: usize,
}

impl LessonSelectWidget<'_> {
    /// Build the display rows: stage headers interspersed with lesson entries.
    fn build_rows(&self) -> Vec<DisplayRow> {
        let mut rows = Vec::new();
        let mut prev_stage: Option<Stage> = None;

        for lesson in &self.curriculum.lessons {
            // Insert stage header when stage changes
            if prev_stage != Some(lesson.stage) {
                if prev_stage.is_some() {
                    rows.push(DisplayRow::Blank);
                }

                let info = lesson.stage.info();

                // Check if all lessons in this stage are completed
                let stage_lessons = self.curriculum.lessons_for_stage(lesson.stage);
                let stage_complete = stage_lessons
                    .iter()
                    .all(|l| l.id < self.highest_unlocked);

                rows.push(DisplayRow::StageHeader {
                    name: info.name,
                    cognitive_goal: info.cognitive_goal,
                    wpm_range: info.wpm_range,
                    stage_complete,
                });

                prev_stage = Some(lesson.stage);
            }

            rows.push(DisplayRow::LessonEntry {
                lesson_id: lesson.id,
                name: lesson.name,
                description: lesson.description,
                is_locked: lesson.id > self.highest_unlocked,
                is_completed: lesson.id < self.highest_unlocked,
            });
        }

        rows
    }

    /// Find the display row index for a given lesson selection index.
    fn display_index_for_selection(rows: &[DisplayRow], selection: usize) -> usize {
        for (i, row) in rows.iter().enumerate() {
            if let DisplayRow::LessonEntry { lesson_id, .. } = row {
                if *lesson_id == selection {
                    return i;
                }
            }
        }
        0
    }
}

impl Widget for LessonSelectWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(2), // header
            Constraint::Min(5),   // lesson list
            Constraint::Length(2), // help
        ])
        .split(area);

        // Header
        let header = Paragraph::new(Line::from(Span::styled(
            " Select a Lesson",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        header.render(chunks[0], buf);

        // Build display rows
        let rows = self.build_rows();
        let selected_display_idx = Self::display_index_for_selection(&rows, self.selected);

        // Lesson list — fixed left margin
        let left_margin = (area.width.saturating_sub(70) / 2).max(2);
        let list_area = Rect {
            x: chunks[1].x + left_margin,
            width: chunks[1].width.saturating_sub(left_margin),
            ..chunks[1]
        };

        let visible_height = list_area.height as usize;

        // Scroll so selected item is visible
        let scroll_offset = if selected_display_idx >= visible_height {
            // Try to center the selection
            selected_display_idx.saturating_sub(visible_height / 2)
        } else {
            0
        };

        let mut lines: Vec<Line> = Vec::new();
        for (_i, row) in rows.iter().enumerate().skip(scroll_offset) {
            if lines.len() >= visible_height {
                break;
            }

            match row {
                DisplayRow::StageHeader {
                    name,
                    cognitive_goal,
                    wpm_range,
                    stage_complete,
                } => {
                    let wpm_str = if wpm_range.1 > 0 {
                        format!(" ({}-{} WPM)", wpm_range.0, wpm_range.1)
                    } else {
                        String::new()
                    };

                    let check = if *stage_complete { "ok " } else { "   " };
                    let check_style = Style::default().fg(Color::Green);

                    lines.push(Line::from(vec![
                        Span::styled(check, check_style),
                        Span::styled(
                            format!("{}{}", name, wpm_str),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]));

                    // Cognitive goal as subtitle (if space)
                    if lines.len() < visible_height {
                        lines.push(Line::from(Span::styled(
                            format!("   {}", cognitive_goal),
                            Style::default().fg(Color::DarkGray),
                        )));
                    }
                }

                DisplayRow::LessonEntry {
                    lesson_id,
                    name,
                    description,
                    is_locked,
                    is_completed,
                } => {
                    let is_selected = *lesson_id == self.selected;
                    let arrow = if is_selected { "> " } else { "  " };

                    let (status, status_style) = if *is_locked {
                        ("--", Style::default().fg(Color::DarkGray))
                    } else if *is_completed {
                        ("ok", Style::default().fg(Color::Green))
                    } else {
                        ("  ", Style::default())
                    };

                    let name_style = if *is_locked {
                        Style::default().fg(Color::DarkGray)
                    } else if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    // Indent lessons under their stage header
                    lines.push(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled(arrow, Style::default().fg(Color::Cyan)),
                        Span::styled(format!("{} ", status), status_style),
                        Span::styled(format!("{:<25}", name), name_style),
                        Span::styled(*description, Style::default().fg(Color::DarkGray)),
                    ]));
                }

                DisplayRow::Blank => {
                    lines.push(Line::from(""));
                }
            }
        }

        let list = Paragraph::new(lines);
        list.render(list_area, buf);

        // Help
        let help = Paragraph::new(Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled("j/k", Style::default().fg(Color::White)),
            Span::styled("] Navigate  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Enter", Style::default().fg(Color::White)),
            Span::styled("] Start  [", Style::default().fg(Color::DarkGray)),
            Span::styled("Esc", Style::default().fg(Color::White)),
            Span::styled("] Back", Style::default().fg(Color::DarkGray)),
        ]))
        .alignment(Alignment::Center);
        help.render(chunks[2], buf);
    }
}
