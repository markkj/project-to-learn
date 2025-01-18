package main

import (
	"fmt"
	"os"
	"syscall"
	"unsafe"
)

var oldState syscall.Termios

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

func main() {
	defer disableRawMode()
	enableRawMode()

	for {
		var buf [1]byte
		os.Stdin.Read(buf[:])
		switch buf[0] {
		case 'q':
			return
		default:
			fmt.Println(buf[0])
		}
	}
}
