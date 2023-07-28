//
// 111. Minimum Depth of Binary Tree
// https://leetcode.com/problems/minimum-depth-of-binary-tree/
//

use crate::structs::tree_node::*;

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut level = vec![root];
        let mut depth = 0i32;
        while !level.is_empty() {
            depth += 1;
            let mut next_level = Vec::new();
            for item in &level {
                if let Some(node) = item {
                    let node = node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        return depth;
                    }
                    if node.left.is_some() {
                        next_level.push(node.left.clone());
                    }
                    if node.right.is_some() {
                        next_level.push(node.right.clone());
                    }
                }
            }
            level = next_level;
        }
        depth
    }
}
