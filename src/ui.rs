use ratatui::{Frame, backend::Backend, layout::Rect, text::Spans, widgets::{Block, Borders, Paragraph}};

pub fn render<B: Backend>(f: &mut Frame<B>, _app: &crate::App) {
    let size = f.size();
    let block = Block::default().title("Journal UI").borders(Borders::ALL);
    f.render_widget(block, size);
    // TODO: render subpanels
}
