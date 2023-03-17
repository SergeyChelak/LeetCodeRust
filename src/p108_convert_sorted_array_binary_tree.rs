//
// 108. Convert Sorted Array to Binary Search Tree
// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
//

use super::structs::tree_node::*;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(&nums[..])
    }

    fn build_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len > 0 {
            let median = len / 2;
            let mut node = TreeNode::new(nums[median]);
            node.left = Self::build_tree(&nums[..median]);
            node.right = Self::build_tree(&nums[median + 1..]);
            node.wrapped()
            //Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}
