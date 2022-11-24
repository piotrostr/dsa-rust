package main

import "testing"

func TestNumIslands_Example1(t *testing.T) {
	grid := [][]byte{
		[]byte("11110"),
		[]byte("11010"),
		[]byte("11000"),
		[]byte("00000"),
	}
	if got, want := numIslands(grid), 1; got != want {
		t.Errorf("numIslands() = %v, want %v", got, want)
	}
}

func TestNumIslands_Example2(t *testing.T) {
	grid := [][]byte{
		[]byte("11000"),
		[]byte("11000"),
		[]byte("00100"),
		[]byte("00011"),
	}
	if got, want := numIslands(grid), 3; got != want {
		t.Errorf("numIslands() = %v, want %v", got, want)
	}
}

func TestNumIslands_Example3(t *testing.T) {
	grid := [][]byte{
		[]byte("111"),
		[]byte("010"),
		[]byte("111"),
	}
	if got, want := numIslands(grid), 1; got != want {
		t.Errorf("numIslands() = %v, want %v", got, want)
	}
}

// benchmark
func BenchmarkNumIslands(b *testing.B) {
	grid := [][]byte{
		[]byte("11110"),
		[]byte("11010"),
		[]byte("11000"),
		[]byte("00000"),
	}
	for i := 0; i < b.N; i++ {
		numIslands(grid)
	}
}
