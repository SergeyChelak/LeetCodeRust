//
// 890. Find and Replace Pattern
// https://leetcode.com/problems/find-and-replace-pattern/
//

pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern_profile = Self::profile(&pattern);
        words
            .iter()
            .filter(|&word| {
                Self::profile(word)
                    .iter()
                    .zip(pattern_profile.iter())
                    .filter(|&(c_word, c_pattern)| c_word == c_pattern)
                    .count()
                    == pattern_profile.len()
            })
            .cloned()
            .collect()
    }

    fn profile(word: &String) -> Vec<i32> {
        let mut id_counter = 1;
        let mut map = vec![0; 26];
        for byte in word.bytes() {
            let idx = (byte - b'a') as usize;
            if map[idx] == 0 {
                map[idx] = id_counter;
                id_counter += 1;
            }
        }
        let mut profile: Vec<i32> = Vec::with_capacity(word.len());
        word.bytes().for_each(|chr| {
            let idx = (chr - b'a') as usize;
            profile.push(map[idx]);
        });
        profile
    }
}
