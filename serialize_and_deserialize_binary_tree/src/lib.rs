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
        let values = traverse(root);
        let values_string = values
            .iter()
            .map(|opt| match opt {
                Some(value) => value.to_string() + ",",
                None => "null".to_string() + ",",
            })
            .collect::<String>();

        return values_string;
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut data = data;
        // handle case of empty data string
        if data == "" || !data.contains(",") {
            return None;
        }

        // pop the last ","
        data.pop();

        let values = data
            .split(",")
            .map(|x| {
                if x == "null" {
                    return None;
                }
                // unwrapping here, can panic if the string is bad
                return Some(x.parse::<i32>().unwrap());
            })
            .collect::<Vec<Option<i32>>>();

        // use assemble function to get the tree from values
        let tree = assemble(values);

        return tree;
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
pub fn traverse(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    // keep a deque queue of nodes to go through
    // pop the nodes from the back, get their values and and append lefts and
    // rights to the deque
    let mut root = root;
    let mut values: Vec<Option<i32>> = vec![];
    let mut queue = VecDeque::new();
    loop {
        match root {
            Some(ref node) => {
                let node = node.borrow();
                values.push(Some(node.val));
                queue.push_front(node.left.clone());
                queue.push_front(node.right.clone());
            }
            None => {
                values.push(None);
            }
        }
        match queue.pop_back() {
            Some(new_root) => root = new_root,
            None => break,
        }
    }
    return values;
}

/// get a reference to a tree
pub fn rc(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref node) => Some(Rc::clone(node)),
        None => None,
    }
}

pub fn left_append(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut res = None;
    match root {
        Some(node) => {
            res = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.as_ref().borrow_mut().left = rc(&res);
        }
        // if None then already none, nothing to do
        None => {}
    }
    return res;
}

pub fn right_append(
    root: &Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut res = None;
    match root {
        Some(node) => {
            res = Some(Rc::new(RefCell::new(TreeNode::new(val))));
            node.as_ref().borrow_mut().right = rc(&res);
        }
        // if None then already none, nothing to do
        None => {}
    }
    return res;
}

/// The assemble function assumes that the string contains in the order as in
/// the traverse function, using the reverse algorithm of traverse in this case -
/// it might not work with other traversal techniques
///
/// number of Nones is important for the algorithm
///
/// in every level of the tree, one None means there will be a branch dying
///
/// how about
///
/// go through values one by one, pop two on each separate tree
///
/// keep the trees for any given level in a queue to be able go back to attach
/// more values
pub fn assemble(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    // handle case of empty
    if values.is_empty() {
        return None;
    }
    let mut values = VecDeque::from(values.clone());
    let mut queue /* : VecDeque<Option<Rc<RefCell<TreeNode>>>>*/ = VecDeque::new();

    // start with root being the last element on the right side
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        values.pop_front().unwrap().unwrap(),
    ))));

    // start with the first value which is always the root value and the root of the tree
    // then, slap in the values for each of the nodes, after slapping in the
    // values move the node to a queue
    //
    // if there is None's it means that the node will not have a value and just
    // leave the None otherwise, if there is value, initialize the node with
    // None's for the left and right nodes and the value for val

    queue.push_back(rc(&root));

    while !queue.is_empty() && !values.is_empty() {
        let node = queue.pop_front().unwrap();

        // pop one, try to assign to left
        match values.pop_front().unwrap() {
            Some(val) => {
                let left = left_append(&node, val);
                queue.push_back(left);
            }
            None => {}
        };

        // pop one, repeat - try to assign to right
        match values.pop_front().unwrap() {
            Some(val) => {
                let right = right_append(&node, val);
                queue.push_back(right);
            }
            None => {}
        }
    }

    return root;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::any::{Any, TypeId};

    #[test]
    fn traversing_works() {
        let tree = get_tree();
        let values = traverse(tree);
        assert_ne!(values.len(), 0);
    }

    #[test]
    fn serialize_works() {
        let codec = Codec::new();
        let tree = get_tree();
        let serialized = codec.serialize(tree.clone());
        let want = String::from("1,2,3,null,null,4,5,null,null,null,null,");
        assert_eq!(serialized, want);
    }

    #[test]
    fn deserialize_works() {
        let codec = Codec::new();
        let serialized = String::from("1,2,3,null,null,4,5,null,null,null,null,");
        let deserialized = codec.deserialize(serialized);
        let want = get_tree();
        assert_eq!(deserialized, want);
    }

    #[test]
    fn everything_works() {
        let codec = Codec::new();
        let tree = get_tree();
        let serialized = codec.serialize(tree.clone());
        assert_eq!(TypeId::of::<String>(), serialized.type_id());
        let deserialized = codec.deserialize(serialized);
        assert_eq!(tree, deserialized);
    }
}
