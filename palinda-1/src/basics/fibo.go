package main

import "fmt"

func fibonacci() func() int {
}

func main() {
	fib := fibonacci()
	for i := 0; i < 10; i++ {
		fmt.Println(fib())
	}
}
