//
// 94. Binary Tree Inorder Traversal
// https://leetcode.com/problems/binary-tree-inorder-traversal/
//

use super::structs::*;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(value) = root else {
            return vec![];
        };
        let node = value.borrow();
        let mut left_travere = Self::inorder_traversal(node.left.clone());
        let mut right_traverse = Self::inorder_traversal(node.right.clone());
        let mut result: Vec<i32> = Vec::new();
        result.append(&mut left_travere);
        result.push(node.val);
        result.append(&mut right_traverse);
        result
    }

    // leetcode doesn't support let-else guards
    /*
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(value) = root {
            let node = value.borrow();
            let mut left_travere = Self::inorder_traversal(node.left.clone());
            let mut right_traverse = Self::inorder_traversal(node.right.clone());
            let mut result: Vec<i32> = Vec::new();
            result.append(&mut left_travere);
            result.push(node.val);
            result.append(&mut right_traverse);
            result
        } else {
            return vec![]
        }
    }
     */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn p94_case1() {
        let node3 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };

        let node2 = TreeNode {
            val: 2,
            left: node3.wrapped(),
            right: None,
        };

        let node1 = TreeNode {
            val: 1,
            left: None,
            right: node2.wrapped(),
        };
        let result = Solution::inorder_traversal(node1.wrapped());
        let expected = vec![1, 3, 2];
        assert_match(&result, &expected);
    }

    #[test]
    fn p94_case2() {
        let node = TreeNode::new(1);
        let root = node.wrapped();
        let result = Solution::inorder_traversal(root);
        assert_match(&result, &vec![1]);
    }

    #[test]
    fn p94_case3() {
        let result = Solution::inorder_traversal(None);
        assert_match(&result, &vec![]);
    }
}
