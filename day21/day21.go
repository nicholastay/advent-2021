package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
	"strconv"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println(os.Args[0] + " <input file>")
		os.Exit(1)
	}

	file, err := os.Open(os.Args[1])
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Positions
	players := make([]int64, 0, 10)

	// Buffered loop by lines
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		sp := strings.Split(scanner.Text(), ": ")
		n, e := strconv.ParseInt(sp[1], 10, 64)
		if e != nil {
			panic(e)
		}
		players = append(players, n)
	}

	scores := make([]int64, len(players))
	i := 0
	turn := int64(1)
	roll := int64(6)
	for {
		// 1-indexed position adjustment
		players[i] = (players[i] + roll - 1) % 10 + 1
		scores[i] += players[i]
		if scores[i] >= 1000 {
			break
		}

		i = (i + 1) % len(players)
		turn++
		roll += 9
	}

	loser := 0
	if i == 0 {
		loser = 1
	}

	fmt.Println("Player", i, "wins")
	fmt.Println("Turn #:", turn, "--", "Dice rolled:", turn * 3)
	fmt.Println("Final scores:", scores)
	fmt.Println("Losing score * Times rolled =", scores[loser] * turn * 3)
}
