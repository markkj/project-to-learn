package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
	"unsafe"
)

var oldState syscall.Termios

func init() {
	// Save the current terminal state
	fd := os.Stdin.Fd()
	if _, _, err := syscall.Syscall(syscall.SYS_IOCTL, fd, uintptr(syscall.TIOCGETA), uintptr(unsafe.Pointer(&oldState))); err != 0 {
		fmt.Fprintln(os.Stderr, "Error getting terminal state:", err)
		os.Exit(1)
	}
}

func enableRawMode() {
	fd := os.Stdin.Fd()
	var raw syscall.Termios
	if _, _, err := syscall.Syscall(syscall.SYS_IOCTL, fd, uintptr(syscall.TIOCGETA), uintptr(unsafe.Pointer(&raw))); err != 0 {
		fmt.Fprintln(os.Stderr, "Error getting terminal state:", err)
		os.Exit(1)
	}

	raw.Iflag &^= syscall.ICRNL | syscall.IXON
	raw.Lflag &^= syscall.ECHO | syscall.ICANON | syscall.ISIG | syscall.IEXTEN
	raw.Cc[syscall.VMIN] = 1
	raw.Cc[syscall.VTIME] = 0

	if _, _, err := syscall.Syscall(syscall.SYS_IOCTL, fd, uintptr(syscall.TIOCSETA), uintptr(unsafe.Pointer(&raw))); err != 0 {
		fmt.Fprintln(os.Stderr, "Error setting terminal state:", err)
		os.Exit(1)
	}
}

func disableRawMode() {
	fd := os.Stdin.Fd()
	if _, _, err := syscall.Syscall(syscall.SYS_IOCTL, fd, uintptr(syscall.TIOCSETA), uintptr(unsafe.Pointer(&oldState))); err != 0 {
		fmt.Fprintln(os.Stderr, "Error restoring terminal state:", err)
		os.Exit(1)
	}
}

func clearScreen() {
	cmd := exec.Command("clear")
	cmd.Stdout = os.Stdout
	cmd.Run()
}

func moveCursor(x, y int) {
	fmt.Printf("\033[%d;%dH", y, x)
}

func main() {
	defer disableRawMode()
	enableRawMode()

	clearScreen()
	x, y := 10, 10
	moveCursor(x, y)
	fmt.Print("X")
	moveCursor(x, y)

	for {
		var buf [1]byte
		os.Stdin.Read(buf[:])
		switch buf[0] {
		case 'q':
			return
		case 27: // Escape sequence
			var seq [2]byte
			os.Stdin.Read(seq[:])
			if seq[0] == '[' {
				switch seq[1] {
				case 'A': // Up
					if y > 1 {
						y--
					}
				case 'B': // Down
					y++
				case 'C': // Right
					x++
				case 'D': // Left
					if x > 1 {
						x--
					}
				}
			}
		}

		clearScreen()
		moveCursor(x, y)
		fmt.Print("X")
		moveCursor(x, y)
	}
}
