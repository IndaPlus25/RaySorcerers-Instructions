package main

import (
	"fmt"
)

func WordCount(s string) map[string]int {
	// You will implement your solution here
	// The current return value is just a hint, you can replace it
	return map[string]int{"word": 1}
}

func main() {
	fmt.Println(WordCount("The quick brown fox jumped over the lazy dog."))
}
