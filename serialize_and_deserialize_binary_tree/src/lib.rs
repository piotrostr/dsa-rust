use std::cell::RefCell;
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

    pub fn serialize(&self, _root: Option<Rc<RefCell<TreeNode>>>) -> String {
        return "".to_string();
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        any::{Any, TypeId},
        cell::RefCell,
        rc::Rc,
    };

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
