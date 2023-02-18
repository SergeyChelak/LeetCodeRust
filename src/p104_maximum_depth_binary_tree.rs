//
// 104. Maximum Depth of Binary Tree
// https://leetcode.com/problems/maximum-depth-of-binary-tree/
//

use crate::structs::tree_node::*;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

type Node =  Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut level = vec![root];
        let mut levels = 0i32;
        loop {
            if level.is_empty() {
                break levels;
            } else {
                levels += 1;
                level = level.iter()
                .fold(Vec::<Node>::new(), |mut acc, item| {
                    if let Some(ref_node) = item {
                        let node = ref_node.borrow();
                        if let Some(left) = &node.left {
                            acc.push(Some(left.clone()));
                        }
                        if let Some(right) = &node.right {
                            acc.push(Some(right.clone()));
                        }
                    }
                    acc
                });
            }
        }
    }
}