//
// 145. Binary Tree Postorder Traversal
// https://leetcode.com/problems/binary-tree-postorder-traversal/
//
use crate::structs::tree_node::*;
use std::rc::Rc;
use std::cell::RefCell;
struct Solution;

impl Solution {
    // left - right - root
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn traverse(root: Option<Rc<RefCell<TreeNode>>>, array: &mut Vec<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                traverse(node.left.clone(), array);
                traverse(node.right.clone(), array);
                array.push(node.val);
            }
        }
        let mut arr = Vec::new();
        traverse(root, &mut arr);
        arr
    }
}