mod app;
mod http;
mod input;
mod modes;
mod persistence;
mod ui;

use app::{App, Mode};
use crossterm::{
    cursor::{SetCursorStyle, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Show)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // Set cursor style based on mode (vim-like behavior)
        let cursor_style = match app.mode {
            Mode::Normal => SetCursorStyle::SteadyBlock,      // Block cursor in normal mode
            Mode::Insert => SetCursorStyle::BlinkingBar,       // Bar cursor in insert mode
            Mode::Command => SetCursorStyle::BlinkingBar,      // Bar cursor in command mode
        };
        execute!(io::stdout(), cursor_style)?;

        // Draw UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle input events
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) if !input::handle_key_event(app, key) => {
                    // Return false means quit
                    return Ok(());
                }
                _ => (),
            }
        }
    }
}
