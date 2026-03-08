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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = app::App::new();

    let result = run_loop(&mut terminal, &mut app);

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

        // Wait for at least one event
        if event::poll(Duration::from_millis(50))? {
            // Drain all pending events to avoid input lag at high WPM
            loop {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            app.handle_key(key);
                        }
                    }
                    _ => {}
                }
                // Check for more events without blocking
                if !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        app.tick();
    }
}
