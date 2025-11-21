use captains_log::{App, monitor, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Start input monitor
    let (tx, rx) = mpsc::channel();
    monitor::start_monitor(tx);

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Handle global shortcuts
                match key.code {
                    KeyCode::F(10) => app.should_quit = true,
                    KeyCode::Enter => {
                        // Add log entry
                        let content = app.textarea.lines().join("\n");
                        if !content.trim().is_empty() {
                            app.add_log(content);
                            // Reset textarea
                            app.textarea = tui_textarea::TextArea::default();
                            app.textarea.set_placeholder_text("Add log entry...");
                            app.textarea.set_block(
                                ratatui::widgets::Block::default()
                                    .borders(ratatui::widgets::Borders::ALL)
                                    .title("Input"),
                            );
                        }
                    }
                    _ => {
                        app.textarea.input(key);
                    }
                }
            }
        }

        // Check monitor events
        let mut activity_count = 0;
        while let Ok(_) = rx.try_recv() {
            app.register_activity();
            activity_count += 1;
        }

        // Update activity stream
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            // Add activity count to stream
            if app.activity_stream.len() >= 100 {
                app.activity_stream.pop_front();
            }
            app.activity_stream.push_back(activity_count);

            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
