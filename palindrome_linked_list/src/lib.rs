/// there is a number of ways to approach this problem
///
/// one way could be to reverse the list and check in/out for equality
/// this would be two passes, O(2n) or something like that
///
/// another way could be to run a palindrome check on every new element
/// since palindrome goes both ways, would only need to check only one value,
/// knowing the length of the tree, can start from the middle and compare
/// still two passes
///
/// the length is not known:
/// * start by iterating and putting all in hash map
///
/// for odd lengths:
/// * get midpoint - (map.len()/2).ceil()
/// * make two pointers, midpoint - 1 and midpoint + 1
/// * substract/increment and compare
///
/// for even lengths
/// * make two pointers without a midpoint
/// * left := map.len() / 2
/// * right := left + 1
/// * substract/increment and compare
///
/// hash map lookup is O(1)
///
/// array is also a O(1) if you know the index, and for some reason seems more
/// natural to me but would create another structure to keep track of; would
/// pose memory storage tradeoff
pub struct Solution;

use std::collections::HashMap;

// Definition for singly-linked list.
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

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut map: HashMap<usize, i32> = HashMap::new();
        let mut index = 0;
        let mut head = head;
        loop {
            match head {
                Some(node) => {
                    map.insert(index, node.val);
                    index += 1;
                    // for memory safety clone the next node into head
                    // this is not necessarily the efficient solution because
                    // every node contains the whole linked list
                    // not sure if under the hood it clones only the next
                    // element with reference or the entire thing in memory
                    // I think the prior but will see during profiling
                    head = node.next.clone();
                }
                None => break,
            }
        }

        let (mut left, mut right): (usize, usize);
        if map.keys().len() % 2 == 0 {
            // even
            left = map.keys().len() / 2;
            right = left + 1;
        } else {
            // odd
            let midpoint = ((map.keys().len() as f32) / 2.).ceil() as usize;
            left = midpoint - 1;
            right = midpoint + 1;
        }

        let (mut l, mut r): (i32, i32);
        while left != 0 {
            l = *map.get(&left).unwrap();
            r = *map.get(&right).unwrap();
            if l != r {
                return true;
            }
            left -= 1;
            right += 1;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_example_1() {
        // [1, 2, 2, 1]
        let inp_head = Option::Some(Box::new(ListNode {
            val: 1,
            next: Option::Some(Box::new(ListNode {
                val: 2,
                next: Option::Some(Box::new(ListNode {
                    val: 2,
                    next: Option::Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::is_palindrome(inp_head), true);
    }
}
