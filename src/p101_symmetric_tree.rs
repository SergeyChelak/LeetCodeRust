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
            let l_traverse = Self::bf_search(node.left.clone(), true);
            let r_traverse = Self::bf_search(node.right.clone(), false);
            if l_traverse.len() != r_traverse.len() {
                false
            } else {
                l_traverse
                    .iter()
                    .zip(r_traverse.iter())
                    .fold(true, |acc, (a, b)| acc && a == b)
            }
        } else {
            true
        }
    }

    fn bf_search(node: Node, ltr: bool) -> Vec<Option<i32>> {
        let mut current_level: Vec<Node> = vec![node];
        let mut result: Vec<Option<i32>> = Vec::new();
        loop {
            let mut next_level: Vec<Node> = Vec::new();
            for item in current_level.iter() {
                if let Some(ref_cell) = item {
                    let node = ref_cell.borrow();
                    result.push(Some(node.val));
                    if ltr {
                        next_level.push(node.left.clone());
                        next_level.push(node.right.clone());
                    } else {
                        next_level.push(node.right.clone());
                        next_level.push(node.left.clone());
                    }
                } else {
                    result.push(None);
                }
            }
            if next_level.iter().filter(|elem| elem.is_some()).count() > 0 {
                current_level = next_level;
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn p101_case1() {
        //
    }
}
