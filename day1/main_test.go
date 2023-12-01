package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input := `1abc2
	pqr3stu8vwx
	a1b2c3d4e5f
	treb7uchet`

	if res := Part1(input); res != 142 {
		t.Errorf("Part1() = %d, want %d", res, 12345)
	} else {
		t.Log("Part1() = 142")
	}
}

func TestPart2(t *testing.T) {
	input := `two1nine
	eightwothree
	abcone2threexyz
	xtwone3four
	4nineeightseven2
	zoneight234
	7pqrstsixteen`

	if res := Part2(input); res != 281 {
		t.Errorf("Part2() = %d, want %d", res, 12345)
	} else {
		t.Log("Part2() = 281")
	}
}
