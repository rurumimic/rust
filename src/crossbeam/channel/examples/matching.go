// Source:
//   - https://web.archive.org/web/20171209034309/https://www.nada.kth.se/~snilsson/concurrency
//   - http://www.nada.kth.se/~snilsson/concurrency/src/matching.go
//
// Copyright & License:
//   - Stefan Nilsson
//   - Creative Commons Attribution 3.0 Unported License
//   - https://creativecommons.org/licenses/by/3.0/
//

package main

import (
	"fmt"
	"sync"
)

func main() {
	people := []string{"Anna", "Bob", "Cody", "Dave", "Eva"}
	match := make(chan string, 1) // Make room for one unmatched send.

	wg := new(sync.WaitGroup)
	for _, name := range people {
		wg.Add(1)
		go Seek(name, match, wg)
	}

	wg.Wait()

	select {
	case name := <-match:
		fmt.Printf("No one received %s’s message.\n", name)
	default:
		// There was no pending send operation.
	}
}

// Seek either sends or receives, whichever possible, a name on the match
// channel and notifies the wait group when done.
func Seek(name string, match chan string, wg *sync.WaitGroup) {
	select {
	case peer := <-match:
		fmt.Printf("%s received a message from %s.\n", name, peer)
	case match <- name:
		// Wait for someone to receive my message.
	}
	wg.Done()
}
