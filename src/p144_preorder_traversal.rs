//
// 144. Binary Tree Preorder Traversal
// https://leetcode.com/problems/binary-tree-preorder-traversal/
//
use crate::structs::tree_node::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    // root -> left -> right
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let node = node.borrow();
            let mut arr = Vec::new();
            arr.push(node.val);
            let mut left = Self::preorder_traversal(node.left.clone());
            arr.append(&mut left);
            let mut right = Self::preorder_traversal(node.right.clone());
            arr.append(&mut right);
            arr
        } else {
            vec![]
        }
    }
}