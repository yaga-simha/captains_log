use crate::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // Activity Stream
            Constraint::Min(0),     // Journal + Input
            Constraint::Length(3),  // Footer
        ])
        .split(f.size());

    render_header(f, chunks[0]);
    render_activity_stream(f, app, chunks[1]);
    render_journal_section(f, app, chunks[2]);
    render_footer(f, chunks[3]);

    if app.alert_active {
        render_alert(f);
    }
}

fn render_header(f: &mut Frame, area: Rect) {
    let text = ">>> RATATUI: FOCUS LOCK-IN // DEVELOPER PRODUCTIVITY SUITE <<<";
    let paragraph = Paragraph::new(text)
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(paragraph, area);
}

fn render_activity_stream(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("KEYBOARD ACTIVITY STREAM (BRAILLE-GANTT)")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Render Focus Level Gauge inside the Activity Stream area (bottom right)
    // We'll split the inner area
    let activity_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1), // Gauge
        ])
        .split(inner_area);

    // Braille visualization
    let width = activity_chunks[0].width as usize;
    let height = activity_chunks[0].height as usize;

    // We want to visualize the activity stream.
    // We'll take the last 'width' items from the stream.
    let skip = if app.activity_stream.len() > width {
        app.activity_stream.len() - width
    } else {
        0
    };

    let data: Vec<u32> = app.activity_stream.iter().skip(skip).cloned().collect();

    // Create a string for the paragraph
    // We'll just use one line for now, or repeat it for height
    // To make it look like a "stream", we can just show the dots.

    let mut line_string = String::new();
    // Pad with empty if not enough data
    if data.len() < width {
        for _ in 0..(width - data.len()) {
            line_string.push('⠀');
        }
    }

    for &count in &data {
        let c = match count {
            0 => '⠀',
            1..=2 => '⠠',
            3..=5 => '⠆',
            6..=10 => '⠇',
            _ => '⣿',
        };
        line_string.push(c);
    }

    let mut full_text = String::new();
    // Center vertically or fill?
    // Let's fill the middle line
    for i in 0..height {
        if i == height / 2 {
            full_text.push_str(&line_string);
        } else {
            // Empty lines or faint dots
            // full_text.push_str(&" ".repeat(width));
            // Optional: add some static or decoration
            full_text.push_str(&" ".repeat(width));
        }
        full_text.push('\n');
    }

    let paragraph = Paragraph::new(full_text)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(paragraph, activity_chunks[0]);

    let label = format!("CURRENT FOCUS LEVEL: {:.0}% (LOCKED IN)", app.focus_level);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
        .ratio(app.focus_level / 100.0)
        .label(label);
    f.render_widget(gauge, activity_chunks[1]);
}

fn render_journal_section(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Logs
            Constraint::Length(3), // Input
        ])
        .split(area);

    // Logs
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .map(|log| {
            let local_time: chrono::DateTime<chrono::Local> = chrono::DateTime::from(log.timestamp);
            let content = format!("[{}] {}", local_time.format("%H:%M:%S"), log.content);
            ListItem::new(Line::from(Span::raw(content)))
        })
        .collect();

    let logs_list = List::new(logs).block(
        Block::default()
            .title("JOURNAL LOGS (PERSISTENT)")
            .borders(Borders::ALL),
    );
    f.render_widget(logs_list, chunks[0]);

    // Input
    f.render_widget(app.textarea.widget(), chunks[1]);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let text = "RATATUI TUI | V1.2.0                                CONNECTED | F1: HELP | F5: REFRESH | F10: EXIT";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::TOP));
    f.render_widget(paragraph, area);
}

fn render_alert(f: &mut Frame) {
    let area = centered_rect(60, 20, f.size());
    let block = Block::default().title("ALERT").borders(Borders::ALL).style(
        Style::default()
            .bg(Color::Red)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    );
    let paragraph = Paragraph::new("ACTIVITY LOW!\nSTAY FOCUSED!")
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
