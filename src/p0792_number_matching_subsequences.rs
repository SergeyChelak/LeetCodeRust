//
// 792. Number of Matching Subsequences
// https://leetcode.com/problems/number-of-matching-subsequences/
//

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut char_map: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, chr) in s.char_indices() {
            if let Some(v) = char_map.get_mut(&chr) {
                v.push(i);
            } else {
                let v = vec![i];
                char_map.insert(chr, v);
            };
        }

        let mut count = 0;
        for word in words {
            let mut prev: Option<usize> = None;
            let mut is_subsequence = true;
            for chr in word.chars() {
                if let Some(char_indexes) = char_map.get(&chr) {
                    let search_result = char_indexes.iter().find(|x| {
                        if let Some(prev_idx) = prev {
                            **x > prev_idx
                        } else {
                            true
                        }
                    });
                    if let Some(position) = search_result {
                        prev = Some(*position)
                    } else {
                        is_subsequence = false;
                        break;
                    }
                } else {
                    is_subsequence = false;
                    break;
                }
            }
            if is_subsequence {
                count += 1;
            }
        }
        count
    }
}
