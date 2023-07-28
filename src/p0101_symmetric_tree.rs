//
// 101. Symmetric Tree
// https://leetcode.com/problems/symmetric-tree/
//

use super::structs::tree_node::*;

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref_cell) = root {
            let node = ref_cell.borrow();
            Self::traverse(node.left.clone(), node.right.clone())
        } else {
            true
        }
    }

    fn traverse(left: Node, right: Node) -> bool {
        if let (Some(ref_left), Some(ref_right)) = (&left, &right) {
            let node_left = ref_left.borrow();
            let node_right = ref_right.borrow();
            if node_left.val == node_right.val {
                Self::traverse(node_left.left.clone(), node_right.right.clone())
                    && Self::traverse(node_left.right.clone(), node_right.left.clone())
            } else {
                false
            }
        } else {
            left.is_none() && right.is_none()
        }
    }
}
