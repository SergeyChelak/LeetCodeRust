//
// 171. Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/
//

pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .map(|x| (x as u8 - 'A' as u8) as i32 + 1)
            .rev()
            .fold((0i32, 1i32), |(acc, pow), val| (acc + val * pow, pow * 26))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p171_case0() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
    }

    #[test]
    fn p171_case1() {
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }

    #[test]
    fn p171_case2() {
        assert_eq!(Solution::title_to_number("AZ".to_string()), 52);
    }

    #[test]
    fn p171_case3() {
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn p171_case4() {
        assert_eq!(Solution::title_to_number("ZZ".to_string()), 702);
    }
}
