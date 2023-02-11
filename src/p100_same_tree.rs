use super::structs::*;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (Some(p_ref), Some(q_ref)) = (&p, &q) {
            let p_node = p_ref.borrow();
            let q_node = q_ref.borrow();
            if p_node.val != q_node.val {
                false
            } else if !Self::is_same_tree(p_node.left.clone(), q_node.left.clone()) {
                false
            } else {
                Self::is_same_tree(p_node.right.clone(), q_node.right.clone())
            }
        } else if p.is_none() && q.is_none() {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p100_case1() {
       let tree1 = create_213();
       let tree2 = create_213();
       assert!(Solution::is_same_tree(tree1, tree2));
    }
    #[test]
    fn p100_case2() {
       let tree1 = create_12();
       let tree2 = create_21();
       assert!(!Solution::is_same_tree(tree1, tree2));
    }

    #[test]
    fn p100_case3() {
        let tree1 = create_112();
        let tree2 = create_211();
        assert!(!Solution::is_same_tree(tree1, tree2));
    }

    fn create_213() -> TreeNodeWrapped {
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node1 = TreeNode {
            val: 1,
            left: node2.wrapped(),
            right: node3.wrapped()
        };
        node1.wrapped()
    }

    fn create_21() -> TreeNodeWrapped {
        let node2 = TreeNode::new(2);
        let node1 = TreeNode {
            val: 1,
            left: node2.wrapped(),
            right: None
        };
        node1.wrapped()
    }

    fn create_12() -> TreeNodeWrapped {
        let node2 = TreeNode::new(2);
        let node1 = TreeNode {
            val: 1,
            left: None,
            right: node2.wrapped()
        };
        node1.wrapped()
    }

    fn create_211() -> TreeNodeWrapped {
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(1);
        let node1 = TreeNode {
            val: 1,
            left: node2.wrapped(),
            right: node3.wrapped()
        };
        node1.wrapped()
    }

    fn create_112() -> TreeNodeWrapped {
        let node2 = TreeNode::new(1);
        let node3 = TreeNode::new(2);
        let node1 = TreeNode {
            val: 1,
            left: node2.wrapped(),
            right: node3.wrapped()
        };
        node1.wrapped()
    }

}
