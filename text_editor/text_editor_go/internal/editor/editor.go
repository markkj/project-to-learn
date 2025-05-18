package editor

type Editor struct {
	cursorXPosition int64
	cursorYPosition int64

	txtBuff []string
}

func (e *Editor) InsertCharacter(c rune) {
	if len(e.txtBuff) < int(e.cursorYPosition) {
		e.txtBuff = append(e.txtBuff, "")
	}
	line := e.txtBuff[e.cursorYPosition-1]
	e.txtBuff[e.cursorYPosition-1] = line[:e.cursorXPosition-1] + string(c) + line[e.cursorXPosition-1:]
	e.cursorXPosition += 1
}

func (e *Editor) DeleteCharacter() {
	if e.cursorXPosition-1 >= 1 {
		line := e.txtBuff[e.cursorYPosition-1]
		e.txtBuff[e.cursorYPosition-1] = line[:e.cursorXPosition-2] + line[e.cursorXPosition-1:]
		e.cursorXPosition -= 1
	} else if e.cursorYPosition-1 > 0 {
		e.cursorYPosition -= 1
	}
}

func (e *Editor) NewLine() {
	e.cursorYPosition += 1
	if len(e.txtBuff) < int(e.cursorYPosition) {
		e.txtBuff = append(e.txtBuff, "")
	}
	e.cursorXPosition = 1
}

func (e *Editor) MoveCursorLeft() {
	if e.cursorXPosition > 1 {
		e.cursorXPosition -= 1
	}
}

func (e *Editor) MoveCursorRight() {
	if len(e.txtBuff[e.cursorYPosition-1]) >= int(e.cursorXPosition) {
		e.cursorXPosition += 1
	}
}

func (e *Editor) MoveCursorDown() {
	if len(e.txtBuff) > int(e.cursorYPosition) {
		e.cursorYPosition += 1
		if int(e.cursorXPosition) > len(e.txtBuff[e.cursorYPosition-1]) {
			e.cursorXPosition = int64(len(e.txtBuff[e.cursorYPosition-1])) + 1
		}
	}
}

func (e *Editor) MoveCursorUp() {
	if int(e.cursorYPosition) > 1 {
		e.cursorYPosition -= 1
		if int(e.cursorXPosition) > len(e.txtBuff[e.cursorYPosition-1]) {
			e.cursorXPosition = int64(len(e.txtBuff[e.cursorYPosition-1])) + 1
		}
	}
}

func (e *Editor) GetCurrentPosition() (int64, int64) {
	return e.cursorXPosition, e.cursorYPosition
}

func (e *Editor) GetTextBuffer() []string {
	return e.txtBuff
}

func NewEditor() *Editor {
	return &Editor{
		cursorXPosition: 1,
		cursorYPosition: 1,
		txtBuff:         make([]string, 0, 1000),
	}
}
