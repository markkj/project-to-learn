package main

import (
	"fmt"
	"syscall"
	"unsafe"
)

type Winsize struct {
	Rows uint16
	Cols uint16
	X    uint16
	Y    uint16
}

func main() {
	// Open file descriptor for standard output
	fd := uintptr(syscall.Stdin)

	// Allocate memory for Winsize structure
	ws := &Winsize{}

	// Invoke ioctl syscall
	_, _, err := syscall.Syscall(syscall.SYS_IOCTL, fd, uintptr(syscall.TIOCGWINSZ), uintptr(unsafe.Pointer(ws)))
	if err != 0 {
		fmt.Println("Error:", err.Error())
	} else {
		fmt.Printf("Rows: %d, Cols: %d\n", ws.Rows, ws.Cols)
	}
}
