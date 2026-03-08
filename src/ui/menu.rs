use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

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
            Constraint::Length(6),  // title
            Constraint::Min(10),   // menu items
            Constraint::Length(2), // help
        ])
        .split(area);

        // Title
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

        // Menu items
        let mut lines: Vec<Line> = Vec::new();
        for (i, (name, desc)) in MENU_ITEMS.iter().enumerate() {
            let is_selected = i == self.selected;
            let arrow = if is_selected { " > " } else { "   " };
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
        let menu = Paragraph::new(lines).alignment(Alignment::Center);
        menu.render(chunks[1], buf);

        // Help line
        let help = Paragraph::new(Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("↑↓", Style::default().fg(Color::White)),
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

pub struct LessonSelectWidget<'a> {
    pub lessons: &'a [crate::engine::Lesson],
    pub selected: usize,
    pub highest_unlocked: usize,
}

impl Widget for LessonSelectWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(3), // header
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

        // Lesson list — show a scrolling window around the selected item
        let visible_height = chunks[1].height as usize;
        let scroll_offset = if self.selected >= visible_height {
            self.selected - visible_height + 1
        } else {
            0
        };

        let mut lines: Vec<Line> = Vec::new();
        for (i, lesson) in self.lessons.iter().enumerate().skip(scroll_offset) {
            if lines.len() >= visible_height {
                break;
            }

            let is_selected = i == self.selected;
            let is_locked = i > self.highest_unlocked;

            let arrow = if is_selected { " > " } else { "   " };
            let status = if is_locked {
                "🔒 "
            } else if i < self.highest_unlocked {
                "✓  "
            } else {
                "   "
            };

            let name_style = if is_locked {
                Style::default().fg(Color::DarkGray)
            } else if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(arrow, Style::default().fg(Color::Cyan)),
                Span::raw(status),
                Span::styled(
                    format!("{:2}. {:<25}", i + 1, lesson.name),
                    name_style,
                ),
                Span::styled(lesson.description, Style::default().fg(Color::DarkGray)),
            ]));
        }
        let list = Paragraph::new(lines);
        list.render(chunks[1], buf);

        // Help
        let help = Paragraph::new(Line::from(vec![
            Span::styled("  [", Style::default().fg(Color::DarkGray)),
            Span::styled("↑↓", Style::default().fg(Color::White)),
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
