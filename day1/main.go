package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"unicode"
)

func GetNum(line string) int {
	if strings.HasPrefix(line, "one") {
		return 1
	} else if strings.HasPrefix(line, "two") {
		return 2
	} else if strings.HasPrefix(line, "three") {
		return 3
	} else if strings.HasPrefix(line, "four") {
		return 4
	} else if strings.HasPrefix(line, "five") {
		return 5
	} else if strings.HasPrefix(line, "six") {
		return 6
	} else if strings.HasPrefix(line, "seven") {
		return 7
	} else if strings.HasPrefix(line, "eight") {
		return 8
	} else if strings.HasPrefix(line, "nine") {
		return 9
	}
	return 0
}

func Part1(input string) int {
	var ans = 0

	for _, line := range strings.Split(input, "\n") {
		left := -1
		right := -1
		for _, char := range line {
			if unicode.IsDigit(rune(char)) {
				if left == -1 {
					left = int(char - '0')
				} else {
					right = int(char - '0')
				}
			}
		}

		if right == -1 {
			right = left
		}

		ans += left*10 + right
	}

	return int(ans)
}

func Part2(input string) int {
	var ans = 0

	for _, line := range strings.Split(input, "\n") {
		left := -1
		right := -1

		for i := 0; i < len(line); i++ {
			if unicode.IsDigit(rune(line[i])) {
				if left == -1 {
					left = int(line[i] - '0')
				} else {
					right = int(line[i] - '0')
				}
			}

			if val := GetNum(line[i:]); val != 0 {
				if left == -1 {
					left = val
				} else {
					right = val
				}
			}
		}

		if right == -1 {
			right = left
		}

		ans += left*10 + right
	}

	return int(ans)
}

func main() {
	input, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err.Error())
	}

	// Part 1
	fmt.Println(Part1(string(input)))

	// Part 2
	fmt.Println(Part2(string(input)))
}
