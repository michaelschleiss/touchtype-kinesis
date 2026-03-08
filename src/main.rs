mod app;
mod engine;
mod keyboard;
mod persistence;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = app::App::new();

    // Main event loop
    let result = run_loop(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut app::App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| app.render(frame))?;

        if app.should_quit {
            return Ok(());
        }

        // Poll with a short timeout so we can update animations/timers
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key release events (crossterm 0.28 sends both press and release)
                if key.kind == crossterm::event::KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }

        app.tick();
    }
}
