package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
	"unicode"
)

/* getKeys returns the keys of a map */
func getKeys(myMap map[string]int) []string {
	keys := make([]string, 0, len(myMap))

	for key := range myMap {
		keys = append(keys, key)
	}

	return keys
}

/*
readLines reads a whole file into memory
and returns a slice of its lines.
*/
func readLines(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func part1(input []string) uint64 {
	var sum uint64 = 0
	for _, v := range input {
		// extract first and last digit
		first, last := "", ""
		for i := 0; i < len(v); i++ {
			if unicode.IsDigit(rune(v[i])) {
				first = string(v[i])
				break
			}
		}
		for i := len(v) - 1; i >= 0; i-- {
			if unicode.IsDigit(rune(v[i])) {
				last = string(v[i])
				break
			}
		}
		val, _ := strconv.Atoi(first + last)
		sum += uint64(val)
	}

	return sum
}

/* Finds number words and inserts corresponding digits at found indexes */
func part2(input []string) uint64 {
	digitMap := map[string]int{
		"zero":  0,
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}
	numWords := getKeys(digitMap)
	var sum uint64 = 0
	for _, v := range input {
		found := map[int]int{}
		// find index of number words
		for _, numWord := range numWords {
			idx := strings.Index(v, numWord)
			if idx != -1 {
				found[idx] = digitMap[numWord]
			}
		}

		// find indexes of digits
		for i := 0; i < len(v); i++ {
			ch := rune(v[i])
			if unicode.IsDigit(ch) {
				found[i] = int(ch - '0')
			}
		}
		indexes := []int{}
		for k := range found {
			indexes = append(indexes, k)
		}
		slices.Sort(indexes)
		first := found[indexes[0]]
		last := found[indexes[len(indexes)-1]]
		valStr := fmt.Sprintf("%d%d", first, last)
		val, _ := strconv.Atoi(valStr)
		sum += uint64(val)
	}

	return sum
}

func main() {
	/*
		  sample_input1 := []string{
				"1abc2",
				"pqr3stu8vwx",
				"a1b2c3d4e5f",
				"treb7uchet",
			}
	*/

	/*
		sample_input2 := []string{
			"two1nine",
			"eightwothree",
			"abcone2threexyz",
			"xtwone3four",
			"4nineeightseven2",
			"zoneight234",
			"7pqrstsixteen",
		}
	*/
	input1, err := readLines("input1.txt")
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println("Part1: ", part1(input1))

	input2, err := readLines("input2.txt")
	if err != nil {
		fmt.Println(err)
	}

	fmt.Println("Part2: ", part2(input2))
}
