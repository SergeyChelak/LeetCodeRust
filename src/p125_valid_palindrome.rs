//
// 125. Valid Palindrome
// https://leetcode.com/problems/valid-palindrome/
//

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|x| x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        s == s.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p125_case1() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
    }

    #[test]
    fn p125_case2() {
        assert!(!Solution::is_palindrome("race a car".to_string()));
    }

    #[test]
    fn p125_case3() {
        assert!(Solution::is_palindrome(" ".to_string()));
    }

    #[test]
    fn p125_case4() {
        assert!(!Solution::is_palindrome("0P".to_string()));
    }

    #[test]
    fn p125_case5() {
        assert!(!Solution::is_palindrome("9,8".to_string()));
    }
}
