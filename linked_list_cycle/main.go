package main

type ListNode struct {
	Val  int
	Next *ListNode
}

// The solution to the problem seems very simple, just unpack the list and check
// if the index `pos` exists in the array
func HasCycle(head *ListNode, pos int) bool {
	var arr []int
	for {
		arr = append(arr, head.Val)
		head = head.Next
		if head == nil {
			break
		}
	}

	// no negative indices in the array allowed
	if pos < 0 {
		return false
	}

	// the index cannot be outside of the array
	if (pos - 1) >= len(arr) {
		return false
	}

	return true
}
