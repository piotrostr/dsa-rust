use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::<i32>::new();
        for list in lists {
            let mut list = list;
            while list.is_some() {
                let node = list.unwrap();
                heap.push(node.val);
                list = node.next;
            }
        }

        let mut vals = heap.into_sorted_vec();

        // assemble the tree from the back
        vals.reverse();
        let mut ptr: Option<Box<ListNode>> = None;
        for val in vals {
            let new_node = Some(Box::new(ListNode { val, next: ptr }));
            ptr = new_node;
        }

        ptr
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    /// Input: lists = [[1,4,5],[1,3,4],[2,6]]
    /// Output: [1,1,2,3,4,4,5,6]
    #[test]
    fn example_1() {
        let lists = vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 6, next: None })),
            })),
        ];
        let result = Solution::merge_k_lists(lists);
        let want = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None })),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(result, want);
    }
}
