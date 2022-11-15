package main

import (
	"reflect"
	"testing"
)

func TestFloodFill(t *testing.T) {
	image := [][]int{
		{1, 1, 1},
		{1, 1, 0},
		{1, 0, 1},
	}
	want := [][]int{
		{2, 2, 2},
		{2, 2, 0},
		{2, 0, 1},
	}
	sr := 1
	sc := 1
	color := 2
	got := FloodFill(image, sr, sc, color)
	if !reflect.DeepEqual(got, want) {
		t.Errorf("TestFloodFill: got %v want %v\n", got, want)
	}
}
