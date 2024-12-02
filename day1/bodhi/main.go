package main

import (
	"os"
)

func main() {
	fi, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}

	defer func() {
		if err := fi.Close(); err != nil {
			panic(err)
		}
	}()

	//fuck go for the moment
}
