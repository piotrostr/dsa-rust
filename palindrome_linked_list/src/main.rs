use palindrome_linked_list::{ListNode, Solution};

fn main() {
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
    println!("{}", Solution::is_palindrome(inp_head));
}
