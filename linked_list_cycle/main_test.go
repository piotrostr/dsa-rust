package main

import (
	"testing"
)

func TestHasCycle(t *testing.T) {
	tests := []struct {
		name string
		head *ListNode
		pos  int
		want bool
	}{
		// Example 1:
		// Input: head = [3,2,0,-4], pos = 1
		// Output: true
		// Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).
		{
			name: "Example 1",
			head: &ListNode{
				3, &ListNode{
					2, &ListNode{
						0, &ListNode{
							-4, nil,
						},
					},
				},
			},
			pos:  0,
			want: true,
		},

		// Example 2:
		// Input: head = [1,2], pos = 0
		// Output: true
		// Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.
		{
			name: "Example 2",
			head: &ListNode{
				1, &ListNode{
					2, nil,
				},
			},
			pos:  0,
			want: true,
		},

		// Example 3:
		// Input: head = [1], pos = -1j
		// Output: false
		// Explanation: There is no cycle in the linked list.
		{
			name: "Example 3",
			head: &ListNode{1, nil},
			pos:  -1,
			want: false,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			got := HasCycle(test.head, test.pos)
			if got != test.want {
				t.Fatalf("hasCycle failed: got %v, want %v", got, test.want)
			}
		})
	}
}
