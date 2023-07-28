//
// 112. Path Sum
// https://leetcode.com/problems/path-sum/
//
use std::cell::RefCell;
use std::rc::Rc;

use crate::structs::tree_node::*;
struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::find_sum(root, target_sum, None)
    }

    fn find_sum(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        current_sum: Option<i32>,
    ) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            let mut new_value = root.val;
            if let Some(current_sum) = current_sum {
                new_value += current_sum;
            }
            if new_value == target_sum && root.left.is_none() && root.right.is_none() {
                true
            } else {
                Self::find_sum(root.left.clone(), target_sum, Some(new_value))
                    || Self::find_sum(root.right.clone(), target_sum, Some(new_value))
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p112_test() {
        let root = TreeNode::new(1).wrapped();
        assert!(Solution::has_path_sum(root, 1));
    }
}
