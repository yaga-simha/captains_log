use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
};
use crate::App;

pub fn render(f: &mut Frame, _app: &App) {
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
    let list_block = Block::default().title("List").borders(Borders::ALL);
    let view_block = Block::default().title("View").borders(Borders::ALL);

    f.render_widget(write_block, chunks[0]);
    f.render_widget(list_block, chunks[1]);
    f.render_widget(view_block, chunks[2]);
}
