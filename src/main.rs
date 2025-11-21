use std::error::Error;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{event, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};

mod journal;
mod ui;

struct App {
    // TODO: add app state fields
}

impl App {
    fn new() -> App {
        App { }
    }
}

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
            let evt = event::read()?;
            if let event::Event::Key(key) = evt {
                if key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restore terminal
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
