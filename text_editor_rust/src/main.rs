mod terminal_editor;
use terminal_editor::TerminalEditor;

fn main() {
    let mut te = TerminalEditor::default();
    te.run();
}
