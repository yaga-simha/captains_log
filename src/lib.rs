pub mod journal;
pub mod ui;

pub struct App {
    pub input: String,
    pub cursor_position: usize,
    // TODO: add app state fields
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            cursor_position: 0,
        }
    }
}