package main

import (
	"bufio"
	"fmt"
	"os"
)

func generate(ch chan<- int) {
	for i := 2; ; i++ {
		ch <- i
	}
}

func filter(in <-chan int, out chan<- int, prime int) {
	for i := range in {
		if i%prime != 0 {
			out <- i
		}
	}
}

func main() {
	n := 1000
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()
	ch := make(chan int)
	go generate(ch)
	for range n {
		prime := <-ch
		fmt.Println(out, prime)
		next := make(chan int)
		go filter(ch, next, prime)
		ch = next
	}
}
