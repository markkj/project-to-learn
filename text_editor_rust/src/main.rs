mod terminal_editor;
use terminal_editor::TerminalEditor;

fn main() {
    let te = TerminalEditor::default();
    te.run();
}
