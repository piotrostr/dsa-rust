package main

// how is this graph theory?
//
// the islands are graphs
//
// solution can be obtained through forming out all of the possible graphs of
// ones in the grid
//
// search through all o the
func numIslands(grid [][]byte) int {
	// keep track of the already visited spots
	// visitedMap := map[int]map[int]bool{}
	islandsCount := 0

	// iterate through all of the squares but break if already visited

	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[0]); col++ {
			// for the first time if
			if grid[row][col] == '1' {
				islandsCount += 1
			}

			// if it is not in the map it has not been visited yet
			// run the DFS
			// DFS will stop when it hits a zero, only visiting the
			// island
			dfs(grid, row, col)
		}
	}
	return islandsCount
}

func dfs(grid [][]byte, row int, col int) {
	if grid[row][col] != '1' {
		return
	}

	grid[row][col] = '#'

	// spread around peers, check if won't go out of bounds though

	// go left
	if row-1 >= 0 {
		dfs(grid, row-1, col)
	}
	// go right
	if row+1 < len(grid) {
		dfs(grid, row+1, col)
	}
	// go up
	if col-1 >= 0 {
		dfs(grid, row, col-1)
	}
	// go down
	if col+1 < len(grid[0]) {
		dfs(grid, row, col+1)
	}
}
