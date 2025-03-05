#![warn(clippy::all, clippy::pedantic)]
use editor::Editor;
mod document;
mod editor;
mod row;
mod terminal;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
