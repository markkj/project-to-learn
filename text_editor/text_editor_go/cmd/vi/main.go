package main

import (
	"fmt"

	"github.com/markkj/project-to-learn/text_editor/internal/terminal"
)

func main() {
	defer func() {
		if r := recover(); r != nil {
			fmt.Println("panic!", r)
		}
	}()
	t := terminal.NewTerminal()
	t.Run()
}
