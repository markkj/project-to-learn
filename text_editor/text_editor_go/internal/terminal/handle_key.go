package terminal

import (
	"os"
)

// KeyCode represents different kinds of key events.
type KeyCode int

const (
	KeyUnknown KeyCode = iota
	KeyArrowUp
	KeyArrowDown
	KeyArrowLeft
	KeyArrowRight
	KeyBackSpace
	KeyEnter
	KeyEsc
	KeyCtrlChar
	KeyChar // for normal character keys
)

type KeyEvent struct {
	KeyCode KeyCode
	Char    rune
}

func ParseKeyEvent() (KeyEvent, error) {
	bufKeyPress := make([]byte, 3)
	n, err := os.Stdin.Read(bufKeyPress)
	if err != nil || n == 0 {
		return KeyEvent{}, err
	}

	if bufKeyPress[0] == 27 && bufKeyPress[1] == 0 {
		return KeyEvent{KeyCode: KeyEsc}, nil
	}

	// If it's not an escape character, it's likely a normal key.
	if bufKeyPress[0] != 27 && bufKeyPress[1] == 0 {
		switch bufKeyPress[0] {
		case 13:
			return KeyEvent{KeyCode: KeyEnter}, nil
		case 127:
			return KeyEvent{KeyCode: KeyBackSpace}, nil
		default:
			if bufKeyPress[0] >= 1 && bufKeyPress[0] <= 16 {
				return KeyEvent{KeyCode: KeyCtrlChar}, nil
			}
			return KeyEvent{KeyCode: KeyChar, Char: rune(bufKeyPress[0])}, nil
		}
	}

	// Check for arrow keys.
	if bufKeyPress[0] == 27 && bufKeyPress[1] == 91 {
		switch bufKeyPress[2] {
		case 65:
			return KeyEvent{KeyCode: KeyArrowUp}, nil
		case 66:
			return KeyEvent{KeyCode: KeyArrowDown}, nil
		case 67:
			return KeyEvent{KeyCode: KeyArrowRight}, nil
		case 68:
			return KeyEvent{KeyCode: KeyArrowLeft}, nil
		default:
			return KeyEvent{KeyCode: KeyUnknown}, nil
		}
	}

	return KeyEvent{}, nil
}
