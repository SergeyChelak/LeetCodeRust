//
// 66. Plus One
// https://leetcode.com/problems/plus-one/
//

pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut array = digits.clone();
        for i in (0..array.len()).rev() {
            if array[i] < 9 {
                array[i] += 1;
                return array;
            } else {
                array[i] = 0;
            }
        }
        array.insert(0, 1);
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn p66_test() {
        struct TestCase {
            input: Vec<i32>,
            output: Vec<i32>,
        }
        let test_cases = [
            TestCase {
                input: Vec::from([1, 2, 3]),
                output: Vec::from([1, 2, 4]),
            },
            TestCase {
                input: Vec::from([4, 3, 2, 1]),
                output: Vec::from([4, 3, 2, 2]),
            },
            TestCase {
                input: Vec::from([9]),
                output: Vec::from([1, 0]),
            },
        ];
        for data in test_cases {
            let result = Solution::plus_one(data.input.clone());
            assert_eq!(result, data.output);
        }
    }
}
