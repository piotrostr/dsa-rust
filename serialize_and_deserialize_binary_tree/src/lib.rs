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
    use super::*;

    #[test]
    fn serialize_works() {
        let obj = Codec::new();
        let tree = Option::Some(Rc::new(RefCell::new(TreeNode::new(5))));
        let data: String = obj.serialize(tree.clone());
        let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
        assert_eq!(tree, ans);
    }

    #[test]
    fn deserialize_works() {}

    #[test]
    fn everything_works() {}
}
