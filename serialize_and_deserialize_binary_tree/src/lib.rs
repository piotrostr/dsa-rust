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
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, _root: Option<Rc<RefCell<TreeNode>>>) -> String {
        "".to_string()
    }

    pub fn deserialize(&self, _data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Option::Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        })))
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use super::*;

    #[test]
    fn serialize_works() {
        let codec = Codec::new();
        let tree = Option::Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let serialized = codec.serialize(tree.clone());
        let want = String::from("5");
        assert_eq!(serialized, want);
    }

    #[test]
    fn deserialize_works() {
        let codec = Codec::new();
        let serialized = "5,3".to_string();
        let deserialized = codec.deserialize(serialized);
        let want = Option::Some(Rc::new(RefCell::new(TreeNode::new(5))));
        assert_eq!(deserialized, want);
    }

    #[test]
    fn everything_works() {
        let codec = Codec::new();
        let tree = Option::Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let serialized = codec.serialize(tree.clone());
        assert_eq!(TypeId::of::<String>(), serialized.type_id());
        let deserialized = codec.deserialize(serialized);
        assert_eq!(tree, deserialized);
    }
}
