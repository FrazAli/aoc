package main

import (
	"fmt"
	"regexp"
	"strconv"

	"github.com/FrazAli/aoc/2023/utils"
)

/*
Had to read up on 'Pick's Theorem', 'Shoelace Formula' and 'Green's Theorem'
https://en.wikipedia.org/wiki/Pick%27s_theorem
https://en.wikipedia.org/wiki/Shoelace_formula
https://en.wikipedia.org/wiki/Green%27s_theorem
*/

type position struct {
	x, y int
}

type direction position

type instruction struct {
	dir  direction
	dist int
}

// directions when starting at (0,0) from top left
var directions = map[string]direction{
	"0": {1, 0},
	"1": {0, 1},
	"2": {-1, 0},
	"3": {0, -1},
}

var directionsNum = map[string]direction{
	"R": {1, 0},
	"L": {-1, 0},
	"U": {0, -1},
	"D": {0, 1},
}

func part1(input []string) {
	re := regexp.MustCompile(`^([RLUD]) ([0-9]+) \((#[0-9a-f]{6})\)$`)
	var instructions []instruction
	for i := 0; i < len(input); i++ {
		match := re.FindStringSubmatch(input[i])
		// fmt.Println(match)
		dir := match[1]
		dist, _ := strconv.Atoi(match[2])
		// color := match[3]
		// fmt.Println(dir, dist, color)
		// collect instructions i.e. direction and distance
		instructions = append(instructions, instruction{directions[dir], dist})
	}

	fmt.Println("Part1: ", dig(instructions))
}

func part2(input []string) {
	re := regexp.MustCompile(`^([RLUD]) ([0-9]+) \(#([0-9a-f]{6})\)$`)
	var instructions []instruction
	for i := 0; i < len(input); i++ {
		match := re.FindStringSubmatch(input[i])
		// fmt.Println(match)
		distDir := match[3]                               // e.g. [#]1f1f10
		dir := distDir[5:6]                               // e.g. 0
		dist, _ := strconv.ParseInt(distDir[0:5], 16, 64) // e.g. 1f1f1 i.e. 127473
		// fmt.Println(dir, dist, color)
		// collect instructions i.e. direction and distance
		instructions = append(instructions, instruction{directions[dir], int(dist)})
	}

	fmt.Println("Part2: ", dig(instructions))
}

func dig(instructions []instruction) uint64 {
	points := []position{{0, 0}} // Initial position at top left
	boundary := 0
	current := position{0, 0}
	for _, inst := range instructions {
		current.x += inst.dir.x * inst.dist
		current.y += inst.dir.y * inst.dist
		points = append(points, position{current.x, current.y})
		boundary += inst.dist
	}

	return area(points) + uint64(boundary/2) + 1
}

func area(points []position) uint64 {
	// close polygon i.e. circle back to initial position
	points = append(points, points[0])

	area := uint64(0)
	for i := 0; i < len(points)-1; i++ {
		area += shoelace(points[i], points[i+1])
	}

	return area / 2
}

func shoelace(p1, p2 position) uint64 {
	x1 := uint64(p1.x)
	y1 := uint64(p1.y)
	x2 := uint64(p2.x)
	y2 := uint64(p2.y)
	return (x1 * y2) - (y1 * x2)
}

func main() {
	input, err := utils.ReadLines("input.txt")
	if err != nil {
		fmt.Println(err)
	}

	part1(input)
	part2(input)
}
