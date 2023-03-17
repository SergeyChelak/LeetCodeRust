//
// 34. Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
//

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let from = Self::binary_search(&nums, target, true);
        let to = Self::binary_search(&nums, target, false);
        vec![from, to]
    }

    fn binary_search(nums: &Vec<i32>, target: i32, is_left: bool) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;
        let mut i = -1;
        while l <= r {
            let mid = (l + r) / 2;
            let value = nums[mid as usize];
            if value < target {
                l = mid + 1;
            } else if value > target {
                r = mid - 1;
            } else {
                i = mid;
                if is_left {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn p34_test() {
        let input = vec![5, 7, 7, 8, 8, 10];
        let output = Solution::search_range(input, 8);
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], 3);
        assert_eq!(output[1], 4);
    }
}
