#![warn(clippy::all, clippy::pedantic)]
use editor::Editor;

mod editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
