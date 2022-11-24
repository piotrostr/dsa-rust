package main

import "fmt"

const One = 0b110001

// how is this graph theory?
//
// the islands are graphs
//
// solution can be obtained through forming out all of the possible graphs of
// ones in the grid
func numIslands(grid [][]byte) int {
	// keep track of the already visited spots
	// visitedMap := map[int]map[int]bool{}
	visitedMap := makeMap(len(grid), len(grid[0]))
	islandsCount := 0

	// iterate through all of the squares but break if already visited

	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[0]); col++ {
			// for the first time if
			if isValid(grid[row][col]) && !wasVisited(row, col, visitedMap) {
				islandsCount += 1
			}

			// if it is not in the map it has not been visited yet run the DFS
			// DFS will stop when it hits a zero, only visiting the island
			dfs(grid, row, col, visitedMap)
		}
	}
	fmt.Printf("%+v", visitedMap)
	return islandsCount
}

func makeMap(n int, m int) map[int]map[int]bool {
	visitedMap := make(map[int]map[int]bool)
	for i := 0; i < n; i++ {
		visitedMap[i] = make(map[int]bool)
		for j := 0; j < m; j++ {
			visitedMap[i][j] = false
		}
	}
	return visitedMap
}

func dfs(grid [][]byte, row int, col int, visitedMap map[int]map[int]bool) {
	visited := wasVisited(row, col, visitedMap)
	if visited {
		return
	}

	visitedMap[row][col] = true

	if !isValid(grid[row][col]) {
		return
	}

	// spread around peers, check if won't go out of bounds though
	// go left
	if row-1 > 0 {
		dfs(grid, row-1, col, visitedMap)
	}
	// go right
	if row+1 < len(grid)-1 {
		dfs(grid, row+1, col, visitedMap)
	}
	// go up
	if col-1 > 0 {
		dfs(grid, row, col-1, visitedMap)
	}
	// go down
	if col+1 < len(grid[0])-1 {
		dfs(grid, row, col+1, visitedMap)
	}
}

func wasVisited(row int, col int, visitedMap map[int]map[int]bool) bool {
	visited, ok := visitedMap[row][col]
	if !ok {
		visited = false
	}

	return visited
}

func isValid(b byte) bool {
	if b == One {
		return true
	}

	return false
}
