package main

import (
	"os"
	"regexp"
	"strconv"
	"strings"
)

func main() {
	file, _ := os.ReadFile(os.Args[1])
	fileString := string(file)

	pattern := `mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)`
	re := regexp.MustCompile(pattern)
	matches := re.FindAllStringSubmatch(fileString, -1)

	total := 0
	partOneTotal := 0
	enabled := true
	for _, match := range matches {
		if strings.Contains(match[0], "do()") {
			enabled = true
			continue
		} else if strings.Contains(match[0], "don't()") {
			enabled = false
			continue
		}
		numOne, _ := strconv.Atoi(match[1])
		numTwo, _ := strconv.Atoi(match[2])

		partOneTotal += numOne * numTwo
		if enabled {
			total += numOne * numTwo
		}
	}

	println(partOneTotal) // PART ONE
	println(total)        // PART TWO

}
