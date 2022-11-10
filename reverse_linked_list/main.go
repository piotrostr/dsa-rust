package main

type ListNode struct {
	Val  int
	Next *ListNode
}

// this has complexity O(2n)
func ReverseList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	arr := []*ListNode{}
	for head != nil {
		arr = append(arr, head)
		head = head.Next
	}

	for i := len(arr) - 1; i >= 0; i-- {
		if i == 0 {
			arr[i].Next = nil
			break
		}
		arr[i].Next = arr[i-1]
	}

	return arr[len(arr)-1]
}
