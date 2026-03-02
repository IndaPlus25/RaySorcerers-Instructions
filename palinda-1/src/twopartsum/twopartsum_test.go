package main

import (
	"testing"
)

func TestConcurrentsum(t *testing.T) {
	t.Parallel()
	// TODO add at least two more test cases!
	tests := map[string]struct {
		input  []int
		output int
	}{
		"sum even array": {
			input:  []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10},
			output: 55,
		},
	}

	// run the table of tests that currently consists of an array with an even amount of elements
	for name, test := range tests {
		t.Run(name, func(t *testing.T) {
			t.Parallel()
			if actual, expected := ConcurrentSum(test.input), test.output; actual != expected {
				t.Errorf("Test: %q failed: expected %d, got %d", name, expected, actual)
			}
		})
	}
}
