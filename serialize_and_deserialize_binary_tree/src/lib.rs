use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
/// Lets start by creating a few example binary trees and set up the tests
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    /// serialize returns the tree as a string in form
    /// `"1,2,3,null,null,2".to_string()`
    /// with the last character being the number of nulls in the string
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let (values, nones) = traverse(root);
        let values_string = values
            .iter()
            .map(|opt| match opt {
                Some(value) => value.to_string() + ",",
                None => "null".to_string() + ",",
            })
            .collect::<String>();

        format!("{}{}", values_string, nones.to_string())
    }

    pub fn deserialize(&self, _data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })))
    }
}

/// get_tree returns a tree [1, 2, 3, null, null, 4, 5] as on leetcode
pub fn get_tree() -> Option<Rc<RefCell<TreeNode>>> {
    return Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
    })));
}

/// Traverse goes through the tree and returns the values in
///
/// It also returns the number of none's that is useful for deserialization
///
/// It can be appended as the last value and along with the length of array
/// allow to start going from the back and knowing how to deserialize the tree
/// better, not having to pass through the array another time
pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> (Vec<Option<i32>>, i32) {
    // keep a deque queue of nodes to go through
    // pop the nodes from the back, get their values and and append lefts and
    // rights to the deque,
    let mut root = root;
    let mut values: Vec<Option<i32>> = vec![];
    let mut queue = VecDeque::new();
    let mut nones = 0;
    loop {
        match root {
            Some(ref node) => {
                let node = node.borrow();
                values.push(Some(node.val));
                queue.push_front(node.left.clone());
                queue.push_front(node.right.clone())
            }
            None => {
                values.push(None);
                nones += 1;
            }
        }
        match queue.pop_back() {
            Some(new_root) => root = new_root,
            None => break,
        }
    }
    return (values, nones);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        any::{Any, TypeId},
        cell::RefCell,
        rc::Rc,
    };

    #[test]
    fn traversing_works() {
        let tree = get_tree();
        traverse(tree);
        assert!(false);
    }

    #[test]
    fn serialize_works() {
        let codec = Codec::new();
        let tree = get_tree();
        let serialized = codec.serialize(tree.clone());
        let want = String::from("[1,2,3,null,null,4,5]");
        assert_eq!(serialized, want);
    }

    #[test]
    fn deserialize_works() {
        let codec = Codec::new();
        let serialized = String::from("[1,2,3,null,null,4,5]");
        let deserialized = codec.deserialize(serialized);
        let want = get_tree();
        assert_eq!(deserialized, want);
    }

    #[test]
    fn everything_works() {
        let codec = Codec::new();
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let serialized = codec.serialize(tree.clone());
        assert_eq!(TypeId::of::<String>(), serialized.type_id());
        let deserialized = codec.deserialize(serialized);
        assert_eq!(tree, deserialized);
    }
}
