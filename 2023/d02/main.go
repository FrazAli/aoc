package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/FrazAli/aoc/2023/utils"
)

func part2(input []string) {
	var result uint64 = 0

	for i := 0; i < len(input); i++ {
		maxCubes := map[string]int{
			"red":   1,
			"green": 1,
			"blue":  1,
		}
		splitted := strings.Split(input[i], ":")
		cube_sets := strings.Split(splitted[1], ";")
		for _, cube_set := range cube_sets {
			cubes := strings.Split(cube_set, ",")
			for _, cube := range cubes {
				cube = strings.TrimSpace(cube)
				splitted := strings.Split(cube, " ")
				color := splitted[1]
				cube_number, _ := strconv.Atoi(splitted[0])
				if maxCubes[color] < cube_number {
					maxCubes[color] = cube_number
				}
			}
		}

		// fmt.Println(maxCubes)
		gamePower := maxCubes["red"] * maxCubes["green"] * maxCubes["blue"]
		result += uint64(gamePower)
		// fmt.Println(gamePower)
	}

	// fmt.Println("Valid: ", validGames)
	// fmt.Println("Invalid Games: ", invalidGames)
	fmt.Println("Part2: ", result)
}

func part1(input []string) {
	var validGames []uint64
	var invalidGames []uint64
	var result uint64 = 0
	maxCubes := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}

	for i := 0; i < len(input); i++ {
		gameInvalid := false
		splitted := strings.Split(input[i], ":")
		game_number, _ := strconv.Atoi(strings.Split(splitted[0], " ")[1])
		cube_sets := strings.Split(splitted[1], ";")
		for _, cube_set := range cube_sets {
			cubes := strings.Split(cube_set, ",")
			for _, cube := range cubes {
				cube = strings.TrimSpace(cube)
				splitted := strings.Split(cube, " ")
				color := splitted[1]
				cube_number, _ := strconv.Atoi(splitted[0])
				if maxCubes[color] < cube_number {
					gameInvalid = true
					break
				}
			}

			if gameInvalid {
				break
			}
		}

		if !gameInvalid {
			result += uint64(game_number)
			validGames = append(validGames, uint64(game_number))
		} else {
			invalidGames = append(invalidGames, uint64(game_number))
		}
	}

	// fmt.Println("Valid: ", validGames)
	// fmt.Println("Invalid Games: ", invalidGames)
	fmt.Println("Part1: ", result)
}

func main() {
	// input, err := utils.ReadLines("input.txt")
	input, err := utils.ReadLines("input.txt")
	if err != nil {
		fmt.Println(err)
	}

	part1(input)
	part2(input)
}
