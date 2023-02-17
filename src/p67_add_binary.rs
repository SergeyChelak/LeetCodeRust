//
// 67. Add Binary
// https://leetcode.com/problems/add-binary/
//

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let mut carry = 0;
        let mut aPtr = (a_chars.len() - 1) as i32;
        let mut bPtr = (b_chars.len() - 1) as i32;
        let mut result = String::new();
        while aPtr >= 0 || bPtr >= 0 {
            let x = Self::char_to_i32(&a_chars, &aPtr);
            let y = Self::char_to_i32(&b_chars, &bPtr);            
            let mut sum = x + y + carry;
            carry = 0;
            if sum > 1 {
                carry = 1;
                sum -= 2;
            }
            if sum == 0 {
                result.insert(0, '0');
            } else {
                result.insert(0, '1');
            }            
            aPtr -= 1;
            bPtr -= 1;
        }
        if carry > 0 {
            result.insert(0, '1');
        }
        result
    }

    fn char_to_i32(array: &Vec<char>, index: &i32) -> i32 {
        if *index < 0 {
            0
        } else if array[*index as usize] == '1' { 
            1
        } else { 
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn p67_case1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let expected = "100".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn p67_case2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        let expected = "10101".to_string();
        let result = Solution::add_binary(a, b);
        assert_eq!(result, expected);
    }
}