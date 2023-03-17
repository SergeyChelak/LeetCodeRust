//
// 168. Excel Sheet Column Title
// https://leetcode.com/problems/excel-sheet-column-title/
//

struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut num = column_number;
        let mut array = Vec::new();
        while num > 0 {
            let mut m = num % 26;
            num /= 26;
            if m == 0 {
                num -= 1;
                m = 26;
            }
            array.push(m);
        }
        let offset = 'A' as u8;
        array
            .iter()
            .rev()
            .map(|idx| (*idx as u8 + offset - 1) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p168_case0() {
        assert_eq!(Solution::convert_to_title(1), "A");
    }

    #[test]
    fn p168_case1() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }

    #[test]
    fn p168_case2() {
        assert_eq!(Solution::convert_to_title(52), "AZ");
    }

    #[test]
    fn p168_case3() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn p168_case4() {
        assert_eq!(Solution::convert_to_title(702), "ZZ");
    }
}
