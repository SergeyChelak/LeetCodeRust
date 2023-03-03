//
// 203. Remove Linked List Elements
// https://leetcode.com/problems/remove-linked-list-elements/
//
use super::structs::list_node::*;
pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut list = head.clone();
        let mut ptr = &mut list;
        loop {
            match ptr {
                None => break list,
                Some(node) if node.val == val => {
                    *ptr = node.next.take();
                }
                Some(node) => {
                    ptr = &mut node.next;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}