package main

func FloodFill(image [][]int, sr int, sc int, color int) [][]int {
	maxR := len(image) - 1    // max index of row
	maxC := len(image[0]) - 1 // max index of col

	starting := image[sr][sc]

	// set the color in order not to fall in endless recursion
	image[sr][sc] = color

	// flood the fill recursively

	// dont want out of bounds row indices
	if sr+1 <= maxR {
		if starting == image[sr+1][sc] {
			FloodFill(image, sr+1, sc, color)
		}
	}

	// dont want negative row indices
	if sr-1 >= 0 {
		if starting == image[sr-1][sc] {
			FloodFill(image, sr-1, sc, color)
		}
	}

	// dont want out of bounds col indices
	if sc+1 <= maxC {
		if starting == image[sr][sc+1] {
			FloodFill(image, sr, sc+1, color)
		}
	}

	// dont want negative col indices
	if sc-1 >= 0 {
		if starting == image[sr][sc-1] {
			FloodFill(image, sr, sc-1, color)
		}
	}

	return image
}
