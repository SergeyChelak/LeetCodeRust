//
// 916. Word Subsets
// https://leetcode.com/problems/word-subsets/
//

pub struct Solution;

use std::cmp::max;
const FREQ_SIZE: usize = 26;

fn char_freq(word: &str) -> [i32; FREQ_SIZE] {
    let mut freqs = [0_i32; FREQ_SIZE];
    for chr in word.bytes() {
        freqs[(chr - b'a') as usize] += 1;
    }
    freqs
}

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        // build overall character frequencies
        let mut overall_char_freq = [0_i32; FREQ_SIZE];
        for word in words2 {
            let freq = char_freq(&word);
            for i in 0..FREQ_SIZE {
                overall_char_freq[i] = max(overall_char_freq[i], freq[i]);
            }
        }
        // find universe words
        let mut result: Vec<String> = Vec::new();
        'search_loop: for word in words1 {
            let freq = char_freq(&word);
            for i in 0..FREQ_SIZE {
                if freq[i] < overall_char_freq[i] {
                    continue 'search_loop;
                }
            }
            result.push(word.clone());
        }
        result
    }
}
