package main

import (
	"fmt"
	"net"
)

func main() {
	fmt.Println("start")

	const remote = "127.0.0.1:1123"
	conn, err := net.Dial("tcp", remote)
	if err != nil {
		panic(err)
	}

	for {
		buf := make([]byte, 128)
		if _, err := conn.Read(buf); err != nil {
			panic(err)
		}
		fmt.Println(string(buf))

		if _, err := conn.Write([]byte("telnet-user\n")); err != nil {
			panic(err)
		}
		fmt.Println("--")

	}
}
