//
// 171. Excel Sheet Column Number
// https://leetcode.com/problems/excel-sheet-column-number/
//

pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut pow = 1;
        let mut result = 0i32;
        let offset = 'A' as u8;
        for ch in column_title.chars().rev() {            
            let val = ch as u8 - offset + 1;
            result += val as i32 * pow;
            pow *= 26;
        }
        result
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