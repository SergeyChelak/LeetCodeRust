//
// 205. Isomorphic Strings
// https://leetcode.com/problems/isomorphic-strings/
//

pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_map = [-1; 128];
        let mut t_map = [-1; 128];
        let mut nextId = 0;
        for (ch_s, ch_t) in s.bytes().zip(t.bytes()) {
            let (idx_s, idx_t) = (ch_s as usize, ch_t as usize);
            match (s_map[idx_s], t_map[idx_t]) {
                (a, b) if a == -1 && b == -1 => {
                    s_map[idx_s] = nextId;
                    t_map[idx_t] = nextId;
                    nextId += 1;
                }
                (a, b) if a == b => continue,
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p205_case1() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }

    #[test]
    fn p205_case2() {
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn p205_case3() {
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
    }

    #[test]
    fn p205_case4() {
        assert!(!Solution::is_isomorphic(
            "badc".to_string(),
            "baba".to_string()
        ));
    }
}
