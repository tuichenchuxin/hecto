#![warn(clippy::all, clippy::pedantic)]
mod editor;

fn main() {
    editor::Editor::default().run();
}
