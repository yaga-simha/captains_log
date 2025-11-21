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
        .border_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ); // "Glow" with bold

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Split into Chart area and Bottom Info Bar (Gauge + Stats)
    let activity_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Chart
            Constraint::Length(2), // Gauge + Stats (Stacked)
        ])
        .split(inner_area);

    // --- Waveform Visualization (Bars) ---
    let chart_area = activity_chunks[0];
    let width = chart_area.width as usize;
    let height = chart_area.height as usize;

    let data_len = app.activity_stream.len();
    let mut data = Vec::with_capacity(width);

    if data_len < width {
        let padding = width - data_len;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;

        data.extend([0].iter().cycle().take(left_pad));

        data.extend(app.activity_stream.iter().cloned());

        data.extend([0].iter().cycle().take(right_pad));
    } else {
        data.extend(app.activity_stream.iter().skip(data_len - width).cloned());
    }

    // Create smoothed data (rolling sum over 1 second / 4 ticks)
    // This gives us a range of roughly 0-10+ for WPM 0-120+
    let mut smoothed_data = Vec::with_capacity(width);
    for i in 0..width {
        let mut sum = 0;
        for j in 0..4 {
            if i >= j {
                sum += data[i - j];
            }
        }
        smoothed_data.push(sum);
    }

    // Determine max value for scaling
    // Scale to ~500 LPM (8.33 chars/sec).
    // smoothed_data is chars/sec.
    let max_val = smoothed_data.iter().max().cloned().unwrap_or(1).max(9);

    let mut lines = Vec::with_capacity(height);
    let mid = height as f32 / 2.0;

    // Render from top row to bottom row
    for row in 0..height {
        let mut spans = Vec::with_capacity(width);
        let row_top = row as f32;
        let row_bottom = (row + 1) as f32;

        for &val in &smoothed_data {
            // Color logic based on intensity (smoothed chars/sec)
            // 1-500 LPM range requested.
            // 100 LPM ~= 1.6 cps.
            // 300 LPM ~= 5 cps.
            // 500 LPM ~= 8.3 cps.

            let color = match val {
                0 => Color::DarkGray,   // Idle
                1..=2 => Color::Blue,   // < 120 LPM (Slow)
                3..=5 => Color::Green,  // 120-300 LPM (Medium)
                6..=8 => Color::Yellow, // 300-480 LPM (Fast)
                _ => Color::Red,        // > 480 LPM (Very Fast)
            };

            // Waveform logic
            let amplitude = (val as f32 / max_val as f32) * mid;

            let wave_top = mid - amplitude;
            let wave_bottom = mid + amplitude;

            let overlap_top = row_top.max(wave_top);
            let overlap_bottom = row_bottom.min(wave_bottom);
            let overlap = (overlap_bottom - overlap_top).max(0.0);

            let mut c = ' ';

            if overlap > 0.0 {
                if row as f32 + 0.5 < mid {
                    // Top half
                    c = if overlap > 0.875 {
                        '▇'
                    } else if overlap > 0.75 {
                        '▆'
                    } else if overlap > 0.625 {
                        '▅'
                    } else if overlap > 0.5 {
                        '▄'
                    } else if overlap > 0.375 {
                        '▃'
                    } else if overlap > 0.25 {
                        '▂'
                    } else {
                        ' '
                    };
                } else {
                    // Bottom half
                    if overlap > 0.5 {
                        c = '▀';
                    } else {
                        c = ' ';
                    }
                }
                if overlap >= 0.99 {
                    c = '█';
                }
            }

            // Center line decoration
            if c == ' ' && row == height / 2 {
                c = '─';
                spans.push(Span::styled(
                    c.to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
            } else {
                // Use the calculated color for the bar
                let style = if c == ' ' {
                    Style::default()
                } else {
                    Style::default().fg(color).add_modifier(Modifier::BOLD)
                };
                spans.push(Span::styled(c.to_string(), style));
            }
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).block(Block::default().borders(Borders::NONE));
    f.render_widget(paragraph, chart_area);

    // --- Bottom Info Bar (Gauge + Stats) ---
    let info_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Gauge
            Constraint::Length(1), // Stats
        ])
        .split(activity_chunks[1]);

    let label = format!("CURRENT FOCUS LEVEL: {:.0}% (LOCKED IN)", app.focus_level);
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Blue)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
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
        .alignment(ratatui::layout::Alignment::Center); // Centered stats
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
