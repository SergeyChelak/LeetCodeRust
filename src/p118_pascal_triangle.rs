//
// 118. Pascal's Triangle
// https://leetcode.com/problems/pascals-triangle/
//

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let num_rows = num_rows as usize;
        for i in 1..=num_rows {
            let mut row: Vec<i32> = vec![1; i];
            if i > 2 {
                for j in 1..i - 1 {
                    row[j] = result[i - 2][j - 1] + result[i - 2][j];
                }
            }
            result.push(row);
        }
        return result;
    }
}
