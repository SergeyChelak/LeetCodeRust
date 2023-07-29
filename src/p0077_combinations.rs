//
// 77. Combinations
// https://leetcode.com/problems/combinations/
//

pub struct Solution;

struct Combination {
    max: usize,
    targetSize: usize,
    backtrack: Vec<Vec<i32>>,
}

impl Combination {
    fn new(n: i32, k: i32) -> Self {
        Self {
            max: n as usize,
            targetSize: k as usize,
            backtrack: Vec::new(),
        }
    }

    fn generate(&mut self) {
        let mut array = vec![0; self.targetSize];
        self.helper(0, 1, &mut array);
    }

    fn helper(&mut self, pos: usize, value: usize, array: &mut Vec<i32>) {
        if pos == self.targetSize {
            self.backtrack.push(array.clone());
            return;
        }
        if value > self.max {
            return;
        }
        for i in value..=self.max {
            array[pos] = i as i32;
            self.helper(pos + 1, i + 1, array);
        }
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combination = Combination::new(n, k);
        combination.generate();
        combination.backtrack
    }
}
