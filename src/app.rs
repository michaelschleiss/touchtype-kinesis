use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::SeedableRng;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::engine::{self, Lesson, TypingSession};
use crate::keyboard;
use crate::persistence::UserProgress;
use crate::ui;

const EXERCISE_LEN: usize = 120;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    LessonSelect,
    Typing,
    Results,
    Progress,
    Practice,
    Test,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,

    // Keyboard
    pub layout: keyboard::Layout,

    // Menu
    pub menu_selection: usize,

    // Lessons
    pub lessons: Vec<Lesson>,
    pub lesson_selection: usize,

    // Typing session
    pub session: Option<TypingSession>,
    pub current_lesson_id: usize,
    pub last_pressed: Option<(char, bool)>,
    pub flash_frames: u8,

    // Progress
    pub progress: UserProgress,

    // RNG
    rng: rand::rngs::StdRng,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            should_quit: false,
            layout: keyboard::Layout::kinesis360(),
            menu_selection: 0,
            lessons: engine::lesson::all_lessons(),
            lesson_selection: 0,
            session: None,
            current_lesson_id: 0,
            last_pressed: None,
            flash_frames: 0,
            progress: UserProgress::load(),
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Global quit: Ctrl+C
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        match self.screen {
            Screen::Menu => self.handle_menu_key(key),
            Screen::LessonSelect => self.handle_lesson_select_key(key),
            Screen::Typing | Screen::Practice | Screen::Test => self.handle_typing_key(key),
            Screen::Results => self.handle_results_key(key),
            Screen::Progress => self.handle_progress_key(key),
        }
    }

    fn handle_menu_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_selection > 0 {
                    self.menu_selection -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_selection < ui::MainMenuWidget::item_count() - 1 {
                    self.menu_selection += 1;
                }
            }
            KeyCode::Enter => match self.menu_selection {
                0 => {
                    self.lesson_selection = self.progress.highest_lesson;
                    self.screen = Screen::LessonSelect;
                }
                1 => self.start_practice(),
                2 => self.start_test(),
                3 => self.screen = Screen::Progress,
                4 => self.should_quit = true,
                _ => {}
            },
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_lesson_select_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.lesson_selection > 0 {
                    self.lesson_selection -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.lesson_selection < self.lessons.len() - 1 {
                    self.lesson_selection += 1;
                }
            }
            KeyCode::Enter => {
                if self.lesson_selection <= self.progress.highest_lesson {
                    self.start_lesson(self.lesson_selection);
                }
            }
            KeyCode::Esc => self.screen = Screen::Menu,
            _ => {}
        }
    }

    fn handle_typing_key(&mut self, key: KeyEvent) {
        let Some(session) = &mut self.session else {
            return;
        };

        match key.code {
            KeyCode::Esc => {
                self.session = None;
                self.screen = Screen::Menu;
            }
            KeyCode::Backspace => {
                session.backspace();
                self.last_pressed = None;
            }
            KeyCode::Char(c) => {
                if session.is_finished() {
                    return;
                }
                let expected = session.current_char();
                session.type_char(c);
                let correct = expected == Some(c);
                self.last_pressed = Some((c, correct));
                self.flash_frames = 6;

                if session.is_finished() {
                    self.finish_session();
                }
            }
            _ => {}
        }
    }

    fn handle_results_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => {
                let passed = self.check_passed();
                if passed && self.screen == Screen::Results {
                    let next = self.current_lesson_id + 1;
                    if next < self.lessons.len() {
                        self.start_lesson(next);
                    } else {
                        self.screen = Screen::Menu;
                    }
                } else {
                    self.start_lesson(self.current_lesson_id);
                }
            }
            KeyCode::Char('r') => {
                self.start_lesson(self.current_lesson_id);
            }
            KeyCode::Esc => {
                self.session = None;
                self.screen = Screen::Menu;
            }
            _ => {}
        }
    }

    fn handle_progress_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Esc | KeyCode::Char('q')) {
            self.screen = Screen::Menu;
        }
    }

    fn start_lesson(&mut self, id: usize) {
        self.current_lesson_id = id;
        let lesson = &self.lessons[id];
        let text = lesson.generate_text(EXERCISE_LEN, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Typing;
    }

    fn start_practice(&mut self) {
        // Use all alpha + space
        let all_alpha: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyz ".chars().collect();
        let word_list = engine::words::words_for_chars(&all_alpha);
        let text = engine::words::generate_word_text(&word_list, EXERCISE_LEN, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.current_lesson_id = 0;
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Practice;
    }

    fn start_test(&mut self) {
        let all_alpha: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyz ".chars().collect();
        let word_list = engine::words::words_for_chars(&all_alpha);
        let text = engine::words::generate_word_text(&word_list, 300, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.current_lesson_id = 0;
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Test;
    }

    fn finish_session(&mut self) {
        let Some(session) = &self.session else {
            return;
        };

        let wpm = session.net_wpm();
        let accuracy = session.accuracy();
        let passed = accuracy >= self.lessons.get(self.current_lesson_id)
            .map(|l| l.target_accuracy)
            .unwrap_or(0.9);

        self.progress.record_session(
            self.current_lesson_id,
            wpm,
            accuracy,
            passed && self.screen == Screen::Typing,
            &session.per_key_stats,
        );

        self.screen = Screen::Results;
    }

    fn check_passed(&self) -> bool {
        let Some(session) = &self.session else {
            return false;
        };
        let target = self
            .lessons
            .get(self.current_lesson_id)
            .map(|l| l.target_accuracy)
            .unwrap_or(0.9);
        session.accuracy() >= target
    }

    pub fn tick(&mut self) {
        if self.flash_frames > 0 {
            self.flash_frames -= 1;
            if self.flash_frames == 0 {
                self.last_pressed = None;
            }
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        match self.screen {
            Screen::Menu => {
                let widget = ui::MainMenuWidget {
                    selected: self.menu_selection,
                };
                frame.render_widget(widget, area);
            }
            Screen::LessonSelect => {
                let widget = ui::LessonSelectWidget {
                    lessons: &self.lessons,
                    selected: self.lesson_selection,
                    highest_unlocked: self.progress.highest_lesson,
                };
                frame.render_widget(widget, area);
            }
            Screen::Typing | Screen::Practice | Screen::Test => {
                self.render_typing(frame, area);
            }
            Screen::Results => {
                if let Some(session) = &self.session {
                    let lesson_name = self
                        .lessons
                        .get(self.current_lesson_id)
                        .map(|l| l.name)
                        .unwrap_or("Practice");
                    let target_acc = self
                        .lessons
                        .get(self.current_lesson_id)
                        .map(|l| l.target_accuracy)
                        .unwrap_or(0.9);

                    let widget = ui::ResultsWidget {
                        session,
                        lesson_name,
                        passed: session.accuracy() >= target_acc,
                        target_accuracy: target_acc,
                    };
                    frame.render_widget(widget, area);
                }
            }
            Screen::Progress => {
                let widget = ui::ProgressWidget {
                    progress: &self.progress,
                    total_lessons: self.lessons.len(),
                };
                frame.render_widget(widget, area);
            }
        }
    }

    fn render_typing(&self, frame: &mut Frame, area: Rect) {
        let Some(session) = &self.session else {
            return;
        };

        let chunks = Layout::vertical([
            Constraint::Length(1),  // lesson name
            Constraint::Length(1),  // stats
            Constraint::Length(1),  // blank
            Constraint::Min(4),    // typing area
            Constraint::Length(1),  // blank
            Constraint::Length(9),  // keyboard
            Constraint::Length(1),  // finger hint
            Constraint::Length(1),  // help
        ])
        .split(area);

        // Lesson name
        let title = self
            .lessons
            .get(self.current_lesson_id)
            .map(|l| l.name)
            .unwrap_or("Practice");
        let title_line = ratatui::text::Line::from(ratatui::text::Span::styled(
            format!("  {}", title),
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ));
        frame.render_widget(ratatui::widgets::Paragraph::new(title_line), chunks[0]);

        // Stats line
        let stats = ui::stats_line(session);
        frame.render_widget(ratatui::widgets::Paragraph::new(stats), chunks[1]);

        // Typing area
        let typing_widget = ui::TypingAreaWidget::new(session);
        // Add some horizontal padding
        let typing_area = Rect {
            x: chunks[3].x + 2,
            width: chunks[3].width.saturating_sub(4),
            ..chunks[3]
        };
        frame.render_widget(typing_widget, typing_area);

        // Keyboard
        let mut kb = ui::KeyboardWidget::new(&self.layout);
        kb.highlight_char = session.current_char();
        kb.last_pressed = self.last_pressed;
        frame.render_widget(kb, chunks[5]);

        // Finger hint
        if let Some(c) = session.current_char() {
            let hint = ui::finger_hint_line(&self.layout, c);
            frame.render_widget(ratatui::widgets::Paragraph::new(hint), chunks[6]);
        }

        // Help
        let help = ratatui::text::Line::from(vec![
            ratatui::text::Span::styled(
                "  [Esc] Quit to menu",
                ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray),
            ),
        ]);
        frame.render_widget(ratatui::widgets::Paragraph::new(help), chunks[7]);
    }
}
