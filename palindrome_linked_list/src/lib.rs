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

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn is_palindrome_simple(head: Option<Box<ListNode>>) -> bool {
        let mut vector = vec![];
        let mut head = head;
        loop {
            match head {
                Some(node) => {
                    vector.push(node.val);
                    head = node.next;
                }
                None => break,
            }
        }

        let mut _vector = vector.clone();
        _vector.reverse();

        vector == _vector
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vector = vec![];
        let mut head = head;
        loop {
            match head {
                Some(node) => {
                    vector.push(node.val);
                    head = node.next;
                }
                None => break,
            }
        }

        // even easier would be to just check for equality of a reversed array,
        // this is the solution that was hash-map-based to start with
        let (mut left, mut right): (i32, i32);
        if vector.len() % 2 == 0 {
            // even
            left = (vector.len() / 2 - 1) as i32;
            right = (vector.len() / 2 + 1 - 1) as i32;
        } else {
            // odd
            let midpoint = ((vector.len() as f32) / 2.).ceil() as usize;
            left = (midpoint - 1 - 1) as i32;
            right = (midpoint + 1 - 1) as i32;
        }
        // there is a (-1) everywhere as the index starts count from
        // 0 and length from 1

        let (mut l, mut r): (i32, i32);
        while left >= 0 {
            l = vector[left as usize];
            r = vector[right as usize];
            if l != r {
                return false;
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

    #[test]
    fn is_palindrome_example_2() {
        // [1, 1, 2, 1]
        let inp_head = Option::Some(Box::new(ListNode {
            val: 1,
            next: Option::Some(Box::new(ListNode {
                val: 1,
                next: Option::Some(Box::new(ListNode {
                    val: 2,
                    next: Option::Some(Box::new(ListNode { val: 1, next: None })),
                })),
            })),
        }));
        assert_eq!(Solution::is_palindrome(inp_head), false);
    }
}
