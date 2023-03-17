//
// 1074. Number of Submatrices That Sum to Target
// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
//

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let n = matrix[0].len();
        let m = matrix.len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            dp[i][0] = matrix[i][0];
            for j in 1..n {
                dp[i][j] = matrix[i][j] + dp[i][j - 1];
            }
        }
        let mut result = 0;
        for start in 0..n {
            for end in start..n {
                let mut lookup: HashMap<i32, i32> = HashMap::new();
                let mut sum = 0;
                lookup.insert(0, 1);
                for k in 0..m {
                    sum += dp[k][end];
                    if start > 0 {
                        sum -= dp[k][start - 1];
                    }
                    if let Some(value) = lookup.get(&(sum - target)) {
                        result += *value;
                    }
                    let mut count = 0;
                    if let Some(value) = lookup.get(&sum) {
                        count = *value;
                    }
                    lookup.insert(sum, count + 1);
                }
            }
        }
        return result;
    }
}
