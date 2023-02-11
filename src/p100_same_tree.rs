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
    fn p100_case1() {}
}
