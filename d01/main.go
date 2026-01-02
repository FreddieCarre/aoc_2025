package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const TEST_INPUT = `L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

type DialNumber struct {
	value int
	l     int
	r     int
}

type Dial struct {
	pointer DialNumber
	numbers [100]DialNumber
}

func (d *Dial) rotate(i Instruction) int {
	c := 0
	if i.direction > 0 {
		for _ = range i.value {
			d.pointer = d.numbers[d.pointer.r]
			if d.pointer.value == 0 {
				c += 1
			}
		}
	} else {
		for _ = range i.value {
			d.pointer = d.numbers[d.pointer.l]
			if d.pointer.value == 0 {
				c += 1
			}
		}
	}

	return c
}

type Instruction struct {
	direction int
	value     int
}

type Puzzle struct {
	count int
	dial  Dial

	instructions []Instruction
}

func parse(input string) []Instruction {
	lines := strings.Split(input, "\n")
	instructions := make([]Instruction, len(lines))

	for index, line := range lines {
		parts := strings.Split(line, "")

		if len(parts) == 0 {
			continue
		}

		directionStr := parts[0]
		var direction int

		if directionStr == "R" {
			direction = +1
		} else {
			direction = -1
		}

		valueStr := strings.Join(parts[1:], "")
		value, err := strconv.Atoi(valueStr)

		if err != nil {
			panic("Failed to parse int")
		}

		instr := Instruction{
			direction,
			value,
		}

		instructions[index] = instr
	}

	return instructions
}

func dial() Dial {
	numbers := [100]DialNumber{}
	for i := range 100 {
		l := i - 1
		if l < 0 {
			l = 99
		}
		r := i + 1
		if r > 99 {
			r = 0
		}

		number := DialNumber{
			value: i,
			l:     l,
			r:     r,
		}

		numbers[i] = number
	}

	return Dial{
		pointer: numbers[50],
		numbers: numbers,
	}
}

func puzzle(input string) Puzzle {
	i := parse(input)
	c := 0
	d := dial()

	return Puzzle{
		instructions: i,
		dial:         d,
		count:        c,
	}
}

func readInput() string {
	f, err := os.ReadFile("/Users/freddiecarre/projects/aoc/d01/input.txt")

	if err != nil {
		panic(err)
	}

	return string(f)
}

func part1(test bool) string {
	var p Puzzle

	if test {
		p = puzzle(TEST_INPUT)
	} else {
		p = puzzle(readInput())
	}

	for _, i := range p.instructions {
		p.dial.rotate(i)
		if p.dial.pointer.value == 0 {
			p.count += 1
		}
	}

	return fmt.Sprintf("%v\n", p.count)
}

func part2(test bool) string {
	var p Puzzle

	if test {
		p = puzzle(TEST_INPUT)
	} else {
		p = puzzle(readInput())
	}

	c := 0

	for _, i := range p.instructions {
		c += p.dial.rotate(i)
	}

	return fmt.Sprintf("%v\n", c)
}

func main() {
	fmt.Printf("Part 1 test: %s", part1(true))
	fmt.Printf("Part 1: %s", part1(false))
	fmt.Printf("Part 2 test: %s", part2(true))
	fmt.Printf("Part 2: %s", part2(false))
}
