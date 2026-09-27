// Command project prints the SPEC §6 projection of a CAISON file as JSON.
// Used by the cross-implementation check; also handy on its own.
package main

import (
	"encoding/json"
	"fmt"
	"os"

	caison "github.com/nixpt/caison/impl/go"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Fprintln(os.Stderr, "usage: project <file.caison>")
		os.Exit(2)
	}
	src, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	doc, err := caison.Loads(string(src))
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	out, err := json.Marshal(doc)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println(string(out))
}
