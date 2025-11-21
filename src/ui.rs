use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    text::Text,
};
use crate::App;

pub fn render(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Write panel
            Constraint::Length(5), // List panel
            Constraint::Min(0),    // View panel
        ])
        .split(size);

    let write_block = Block::default().title("Write").borders(Borders::ALL);
    let input = Paragraph::new(Text::raw(&app.input))
        .block(write_block);
    f.render_widget(input, chunks[0]);
    f.set_cursor(
        chunks[0].x + app.cursor_position as u16 + 1,
        chunks[0].y + 1,
    );

    let list_block = Block::default().title("List").borders(Borders::ALL);
    let view_block = Block::default().title("View").borders(Borders::ALL);

    f.render_widget(list_block, chunks[1]);
    f.render_widget(view_block, chunks[2]);
}
