use crossterm::{
    event, execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;

use captains_log::{ui, App};



fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // Main event loop
    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        // Input handling
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char(c) => {
                        app.input.insert(app.cursor_position, c);
                        app.cursor_position += 1;
                    }
                    event::KeyCode::Backspace => {
                        if app.cursor_position > 0 {
                            app.cursor_position -= 1;
                            app.input.remove(app.cursor_position);
                        }
                    }
                    event::KeyCode::Left => {
                        if app.cursor_position > 0 {
                            app.cursor_position -= 1;
                        }
                    }
                    event::KeyCode::Right => {
                        if app.cursor_position < app.input.len() {
                            app.cursor_position += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
