# Build Own Text Editor Guideline

## Ref

- https://github.com/manojVivek/go-vim/tree/master
- https://flenker.blog/hecto/#fnref-1
- ChatGPT

## Things to learn:

- Data structures for storing the text: array, [rope](<https://en.wikipedia.org/wiki/Rope_(data_structure)>), [gap buffer](https://en.wikipedia.org/wiki/Gap_buffer), [piece table](https://en.wikipedia.org/wiki/Piece_table).
- Behavior and implementation of the text cursor.
- Design patterns for undo/redo: [memento](https://en.wikipedia.org/wiki/Memento_pattern), [command](https://en.wikipedia.org/wiki/Command_pattern).
- Abstractions to separate the visual and memory aspects of the text.

## Further reading:

- Text Editor: Data Structures ([web](https://www.averylaird.com/programming/the%20text%20editor/2017/09/30/the-piece-table/))
- Design and Implementation of a Win32 Text Editor ([web](http://www.catch22.net/tuts/neatpad#))
- Data Structures and Algorithms in Java ([Amazon](https://amzn.to/36k1kEv))

# TODO List

## Handling User Input

- Reading keystrokes without pressing Enter (raw mode).
- Handling special keys like arrow keys.

## Text Buffer and Editing

- Implementing an in-memory text buffer.
- Inserting and deleting text efficiently.

## Rendering & UI

- Implementing a simple screen refresh loop.
- Optimizing screen redraws for performance.

## File Handling

- Opening, saving, and handling different encodings.

## Advanced Features

- Undo/redo, syntax highlighting, search, and keybindings.

# Learned

## What is raw mode

By default, the terminal functions in a certain way. For example, it will move the cursor to the beginning of the next line when the input hits the end of a line. Or that the backspace is interpreted for character removal.

Sometimes these default modes are irrelevant, and in this case, we can turn them off. This is what happens when you enable raw modes.

Those modes will be set when enabling raw modes:

- Input will not be forwarded to screen
- Input will not be processed on enter press
- Input will not be line buffered (input sent byte-by-byte to input buffer)
- Special keys like backspace and CTRL+C will not be processed by terminal driver
- New line character will not be processed therefore `println!` can’t be used, use `write!` instead

For entering to raw mode we will use a dependency called `crossterm` to change terminal to receive an input but not print it out

> For more detail about raw mode: https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode

## Escape Sequence

The remaining part, `[2J`, forms part of an **escape sequence**. If you recall our previous discussion about escape sequences, like those for the Arrow Keys, it shouldn't surprise you that they can also instruct the terminal to perform various actions—like clearing the screen.

Here, we’re using the J command ([Erase In Display](http://vt100.net/docs/vt100-ug/chapter3.html#ED)) to wipe the screen clean. The 2 in [2J tells the terminal to clear the entire screen. Different arguments adjust the scope:

- `\x1b[1J` clears up to the cursor.
- `\x1b[0J` clears from the cursor to the end of the screen.
- Simply `\x1b[J` defaults to clearing from the cursor to the end.

For a deeper dive into the magic of terminal commands, explore the [VT100](https://en.wikipedia.org/wiki/VT100) escape sequences widely supported by modern terminals. Full details are in the [VT100 User Guide](http://vt100.net/docs/vt100-ug/chapter3.html).
