use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

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

    fn assert_match(arr1: &Vec<i32>, arr2: &Vec<i32>) {
        arr1.iter()
            .zip(arr2.iter())
            .for_each(|(a, b)| assert_eq!(a, b, "values doesn't match"))
    }

    fn wrap_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn p94_case1() {
        let node3 = TreeNode {
            val: 3,
            left: None,
            right: None,
        };

        let node2 = TreeNode {
            val: 2,
            left: wrap_node(node3),
            right: None,
        };

        let node1 = TreeNode {
            val: 1,
            left: None,
            right: wrap_node(node2),
        };
        let root = wrap_node(node1);
        let result = Solution::inorder_traversal(root);
        let expected = vec![1, 3, 2];
        assert_match(&result, &expected);
    }

    #[test]
    fn p94_case2() {
        let node = TreeNode::new(1);
        let root = wrap_node(node);        
        let result = Solution::inorder_traversal(root);
        assert_match(&result, &vec![1]);
    }

    type NodeLink = Option<Rc<RefCell<TreeNode>>>;
    #[test]
    fn p94_case3() {
        let node = TreeNode::new(1);
        let root: NodeLink = None;
        let result = Solution::inorder_traversal(root);
        assert_match(&result, &vec![]);
   }
}
