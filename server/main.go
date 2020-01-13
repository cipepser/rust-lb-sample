package main

import (
	"fmt"
	"log"
	"net"
	"os"
)

func main() {
	args := os.Args
	if len(args) != 2 {
		log.Fatal("[usage]: go run main.go <LISTEN_PORT>")
	}

	l, err := net.Listen("tcp", "127.0.0.1:"+args[1])
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("listening on", l.Addr(), "...")
	for {
		_, err := l.Accept()
		if err != nil {
			log.Fatal(err)
		}
	}
}
