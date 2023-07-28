//
// 1. Two Sum
// https://leetcode.com/problems/two-sum/
//

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let value = nums[i];
            if let Some(index) = map.get(&(target - value)) {
                return vec![*index, i as i32];
            } else {
                map.insert(value, i as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_match;

    #[test]
    fn p1_tests() {
        // Input: nums = [2,7,11,15], target = 9
        // Output: [0,1]
        assert_match(&Solution::two_sum(vec![2, 7, 11, 15], 9), &vec![0, 1]);

        // Input: nums = [3,2,4], target = 6
        // Output: [1,2]
        assert_match(&Solution::two_sum(vec![3, 2, 4], 6), &vec![1, 2]);

        // Input: nums = [3,3], target = 6
        // Output: [0,1]
        assert_match(&Solution::two_sum(vec![3, 3], 6), &vec![0, 1]);
    }
}
