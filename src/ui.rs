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
    let text = ">>> CAPTAIN'S LOG <<<";
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
        .title("KEYBOARD ACTIVITY")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Split into Chart area and Bottom Info Bar (Gauge + Stats)
    let activity_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Chart
            Constraint::Length(1), // Gauge/Stats
        ])
        .split(inner_area);

    // --- Braille Visualization ---
    let chart_area = activity_chunks[0];
    let width = chart_area.width as usize;
    let height = chart_area.height as usize;

    // Get data for the width of the chart
    // We want to fill from right to left.
    // If we have less data than width, we pad with 0.
    // If we have more, we take the last 'width' items.
    let data_len = app.activity_stream.len();
    let mut data = Vec::with_capacity(width);

    if data_len < width {
        let padding = width - data_len;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;

        for _ in 0..left_pad {
            data.push(0);
        }
        data.extend(app.activity_stream.iter().cloned());
        for _ in 0..right_pad {
            data.push(0);
        }
    } else {
        data.extend(app.activity_stream.iter().skip(data_len - width).cloned());
    }

    // Determine max value for scaling
    let max_val = data.iter().max().cloned().unwrap_or(1).max(5); // Minimum scale of 5

    let mut full_text = String::new();

    // Render from top row to bottom row
    for row in 0..height {
        let mut line_string = String::new();
        // Calculate threshold for this row
        // Row 0 is top, Row height-1 is bottom.
        // We want to map value to height.
        // Normalized value v_norm = v / max_val * height
        // If v_norm >= (height - row), it's full.
        // If v_norm is between (height - row - 1) and (height - row), it's partial.

        for &val in &data {
            let v_norm = (val as f32 / max_val as f32) * height as f32;
            let row_val = height as f32 - row as f32; // e.g. if height=5, row=0 -> 5.0 (top)

            // Check if the bar reaches this row
            if v_norm >= row_val {
                line_string.push('⣿');
            } else if v_norm >= row_val - 1.0 {
                // Partial block
                // Fraction within this block: v_norm - (row_val - 1.0)
                let frac = v_norm - (row_val - 1.0);

                if frac > 0.8 {
                    line_string.push('⣿');
                } else if frac > 0.6 {
                    line_string.push('⠇');
                } else if frac > 0.4 {
                    line_string.push('⠆');
                } else if frac > 0.1 {
                    line_string.push('⠠');
                }
                // minimal dot
                else {
                    line_string.push('⠀');
                }
            } else {
                line_string.push('⠀');
            }
        }
        full_text.push_str(&line_string);
        full_text.push('\n');
    }

    let paragraph = Paragraph::new(full_text)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(paragraph, chart_area);

    // --- Bottom Info Bar (Gauge + Stats) ---
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Gauge
            Constraint::Percentage(30), // Stats
        ])
        .split(activity_chunks[1]);

    let label = format!("CURRENT FOCUS LEVEL: {:.0}% (LOCKED IN)", app.focus_level);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
        .ratio(app.focus_level / 100.0)
        .label(label);
    f.render_widget(gauge, info_chunks[0]);

    let stats_text = format!("WPM: {:03} | LPM: {:04}", app.wpm, app.lpm);
    let stats = Paragraph::new(stats_text)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(ratatui::layout::Alignment::Right);
    f.render_widget(stats, info_chunks[1]);
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
    let block = Block::default().borders(Borders::TOP);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let left_text = "CAPTAIN'S LOG | V1.0.0 | F10: EXIT";
    let right_text = "github: yaga-simha";

    let left_p = Paragraph::new(left_text).style(Style::default().fg(Color::Gray));
    let right_p = Paragraph::new(right_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Right);

    f.render_widget(left_p, inner);
    f.render_widget(right_p, inner);
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
