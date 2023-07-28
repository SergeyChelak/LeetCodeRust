//
// 217. Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/
//

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();
        nums.iter()
            .zip(nums.iter().skip(1))
            .map(|(l, r)| if *l == *r { 1 } else { 0 })
            .max()
            .unwrap_or_default()
            > 0
    }
}
