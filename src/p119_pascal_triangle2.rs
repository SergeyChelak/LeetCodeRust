//
// 119. Pascal's Triangle II
// https://leetcode.com/problems/pascals-triangle-ii/
//

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => vec![1],
            1 => vec![1, 1],
            n => {
                let prev = Self::get_row(n - 1);
                let mut result = vec![1];
                for i in 0..(prev.len() - 1) {
                    result.push(prev[i] + prev[i + 1]);
                }
                result.push(1);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_match;

    #[test]
    fn p119_description_cases() {
        assert_match(&Solution::get_row(3), &vec![1, 3, 3, 1]);
        assert_match(&Solution::get_row(0), &vec![1]);
        assert_match(&Solution::get_row(1), &vec![1, 1]);
    }
}
