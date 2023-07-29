//
// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/
//

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut ptr = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[ptr] = nums[i];
                ptr += 1;
            }
        }
        for i in ptr..nums.len() {
            nums[i] = 0;
        }
    }
}
