// Shared TreeNode struct from LeetCode and custom helper methods
//
pub use std::cell::RefCell;
pub use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub type TreeNodeWrapped = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn wrapped(self) -> TreeNodeWrapped {
        Some(Rc::new(RefCell::new(self)))
    }
}
