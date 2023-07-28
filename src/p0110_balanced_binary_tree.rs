//
// 110. Balanced Binary Tree
// https://leetcode.com/problems/balanced-binary-tree/
//

use std::cell::RefCell;
use std::rc::Rc;

use super::structs::tree_node::*;

struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_balanced = true;
        Self::traverse(root, &mut is_balanced, 0);
        is_balanced
    }

    fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        is_balanced: &mut bool,
        depth: isize,
    ) -> isize {
        if !*is_balanced {
            0
        } else if let Some(ref_root) = root {
            let node = ref_root.borrow();
            let left = Self::traverse(node.left.clone(), is_balanced, depth + 1);
            let right = Self::traverse(node.right.clone(), is_balanced, depth + 1);
            *is_balanced = *is_balanced && (left - right).abs() < 2;
            isize::max(left, right)
        } else {
            depth
        }
    }
}
