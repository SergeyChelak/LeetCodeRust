//
// 202. Happy Number
// https://leetcode.com/problems/happy-number/
//
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut numbers: HashSet<i32> = HashSet::new();
        let mut val = n;
        loop {
            numbers.insert(val);
            let mut tmp = 0;
            while val > 0 {
                let digit = val % 10;
                tmp += digit * digit;
                val /= 10;
            }
            if tmp == 1 {
                break true
            } else if numbers.contains(&tmp) {
                break false
            }
            val = tmp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p202_test() {
        assert!(!Solution::is_happy(2));
        assert!(Solution::is_happy(19));
    }
}