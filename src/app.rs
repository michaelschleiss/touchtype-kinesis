use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::SeedableRng;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::engine::{self, Lesson, TypingSession};
use crate::keyboard;
use crate::persistence::UserProgress;
use crate::ui;

const EXERCISE_LEN: usize = 120;
const MAX_TYPING_WIDTH: u16 = 72;
const MIN_WIDTH: u16 = 50;
const MIN_HEIGHT: u16 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    LessonSelect,
    Typing,
    Results,
    Progress,
}

/// Tracks what mode initiated the typing session, so results can route correctly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingMode {
    Lesson,
    Practice,
    Test,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,

    pub layout: keyboard::Layout,

    pub menu_selection: usize,

    pub lessons: Vec<Lesson>,
    pub lesson_selection: usize,

    pub session: Option<TypingSession>,
    pub current_lesson_id: usize,
    pub typing_mode: TypingMode,
    pub last_pressed: Option<(char, bool)>,
    pub flash_frames: u8,

    pub progress: UserProgress,

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
            typing_mode: TypingMode::Lesson,
            last_pressed: None,
            flash_frames: 0,
            progress: UserProgress::load(),
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        match self.screen {
            Screen::Menu => self.handle_menu_key(key),
            Screen::LessonSelect => self.handle_lesson_select_key(key),
            Screen::Typing => self.handle_typing_key(key),
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
                match self.typing_mode {
                    TypingMode::Lesson => self.screen = Screen::LessonSelect,
                    _ => self.screen = Screen::Menu,
                }
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
                match self.typing_mode {
                    TypingMode::Lesson => {
                        if passed {
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
                    TypingMode::Practice => self.start_practice(),
                    TypingMode::Test => self.start_test(),
                }
            }
            KeyCode::Char('r') => match self.typing_mode {
                TypingMode::Lesson => self.start_lesson(self.current_lesson_id),
                TypingMode::Practice => self.start_practice(),
                TypingMode::Test => self.start_test(),
            },
            KeyCode::Esc => {
                self.session = None;
                self.screen = Screen::Menu;
            }
            _ => {}
        }
    }

    fn handle_progress_key(&mut self, key: KeyEvent) {
        if matches!(key.code, KeyCode::Esc) {
            self.screen = Screen::Menu;
        }
    }

    fn start_lesson(&mut self, id: usize) {
        self.current_lesson_id = id;
        self.typing_mode = TypingMode::Lesson;
        let lesson = &self.lessons[id];
        let text = lesson.generate_text(EXERCISE_LEN, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Typing;
    }

    fn start_practice(&mut self) {
        self.typing_mode = TypingMode::Practice;
        let all_alpha: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyz ".chars().collect();
        let word_list = engine::words::words_for_chars(&all_alpha);
        let text = engine::words::generate_word_text(&word_list, EXERCISE_LEN, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.current_lesson_id = usize::MAX; // no lesson
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Typing;
    }

    fn start_test(&mut self) {
        self.typing_mode = TypingMode::Test;
        let all_alpha: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyz ".chars().collect();
        let word_list = engine::words::words_for_chars(&all_alpha);
        let text = engine::words::generate_word_text(&word_list, 300, &mut self.rng);
        self.session = Some(TypingSession::new(text));
        self.current_lesson_id = usize::MAX;
        self.last_pressed = None;
        self.flash_frames = 0;
        self.screen = Screen::Typing;
    }

    fn finish_session(&mut self) {
        let Some(session) = &self.session else {
            return;
        };

        let wpm = session.net_wpm();
        let accuracy = session.accuracy();
        let target = self
            .lessons
            .get(self.current_lesson_id)
            .map(|l| l.target_accuracy)
            .unwrap_or(0.9);
        let passed = accuracy >= target;

        self.progress.record_session(
            self.current_lesson_id,
            wpm,
            accuracy,
            passed && self.typing_mode == TypingMode::Lesson,
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

        // Minimum terminal size check
        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            let msg = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Terminal too small",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    format!("Need {}x{}, have {}x{}", MIN_WIDTH, MIN_HEIGHT, area.width, area.height),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .alignment(Alignment::Center);
            frame.render_widget(msg, area);
            return;
        }

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
            Screen::Typing => {
                self.render_typing(frame, area);
            }
            Screen::Results => {
                if let Some(session) = &self.session {
                    let lesson_name = self.mode_label();
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

    fn mode_label(&self) -> &str {
        match self.typing_mode {
            TypingMode::Lesson => self
                .lessons
                .get(self.current_lesson_id)
                .map(|l| l.name)
                .unwrap_or("Lesson"),
            TypingMode::Practice => "Practice",
            TypingMode::Test => "Typing Test",
        }
    }

    fn render_typing(&self, frame: &mut Frame, area: Rect) {
        let Some(session) = &self.session else {
            return;
        };

        // Layout: text-dominant, keyboard subordinate
        let chunks = Layout::vertical([
            Constraint::Length(1), // lesson name (dim)
            Constraint::Length(1), // stats
            Constraint::Length(1), // blank
            Constraint::Min(3),   // typing area (gets priority)
            Constraint::Length(1), // blank
            Constraint::Length(5), // keyboard (compact)
            Constraint::Length(1), // finger hint
            Constraint::Length(1), // help
        ])
        .split(area);

        // Lesson name — dim, not attention-grabbing
        let title = self.mode_label();
        let title_line = Line::from(Span::styled(
            format!("  {}", title),
            Style::default().fg(Color::DarkGray),
        ));
        frame.render_widget(Paragraph::new(title_line), chunks[0]);

        // Stats line — centered in its area
        let stats = ui::stats_line(session);
        let stats_area = centered_area(chunks[1], MAX_TYPING_WIDTH);
        frame.render_widget(Paragraph::new(stats), stats_area);

        // Typing area — width-clamped, vertically centered
        let typing_area = centered_area(chunks[3], MAX_TYPING_WIDTH);

        // Estimate lines of text for vertical centering
        let text_chars = session.text.len() as u16;
        let chars_per_line = typing_area.width.max(1);
        let text_lines = (text_chars + chars_per_line - 1) / chars_per_line;
        let vert_offset = typing_area.height.saturating_sub(text_lines) / 2;

        let centered_typing = Rect {
            y: typing_area.y + vert_offset,
            height: typing_area.height.saturating_sub(vert_offset),
            ..typing_area
        };

        let typing_widget = ui::TypingAreaWidget::new(session);
        frame.render_widget(typing_widget, centered_typing);

        // Keyboard — compact, 5 lines
        let mut kb = ui::KeyboardWidget::new(&self.layout);
        kb.highlight_char = session.current_char();
        kb.last_pressed = self.last_pressed;
        frame.render_widget(kb, chunks[5]);

        // Finger hint — centered
        if let Some(c) = session.current_char() {
            let hint = ui::finger_hint_line(&self.layout, c);
            let hint_area = centered_area(chunks[6], MAX_TYPING_WIDTH);
            frame.render_widget(Paragraph::new(hint), hint_area);
        }

        // Help
        let help = Line::from(Span::styled(
            "[Esc] Back",
            Style::default().fg(Color::DarkGray),
        ));
        let help_area = centered_area(chunks[7], MAX_TYPING_WIDTH);
        frame.render_widget(Paragraph::new(help), help_area);
    }
}

/// Create a horizontally centered sub-rect with a max width.
fn centered_area(area: Rect, max_width: u16) -> Rect {
    let width = area.width.min(max_width);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    Rect {
        x,
        width,
        ..area
    }
}
