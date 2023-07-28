//
// 322. Coin Change
// https://leetcode.com/problems/coin-change/
//

use std::collections::HashMap;

trait CoinChange {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32;
}

pub struct Solution;

impl Solution {
    fn coin_change_tabulation(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut amounts = vec![-1; amount + 1];
        amounts[0] = 0;
        for i in 0..=amount as usize {
            if amounts[i] > -1 {
                for coin in coins.iter() {
                    let c = *coin as usize;
                    if i + c > amount {
                        continue;
                    }
                    let old_val = amounts[i + c];
                    let new_val = amounts[i] + 1;
                    amounts[i + c] = if old_val == -1 {
                        new_val
                    } else {
                        old_val.min(new_val)
                    }
                }
            }
        }
        amounts[amount]
    }

    fn coin_change_memo(coins: Vec<i32>, amount: i32) -> i32 {
        fn change(coins: &Vec<i32>, amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if let Some(value) = memo.get(&amount) {
                *value
            } else if amount == 0 {
                0
            } else if amount < 0 {
                -1
            } else {
                let mut result = -1;
                for coin in coins {
                    let mut c = change(coins, amount - coin, memo);
                    if c == -1 {
                        continue;
                    }
                    c += 1;
                    result = if result == -1 { c } else { c.min(result) }
                }
                memo.insert(amount, result);
                result
            }
        }

        let mut memo = HashMap::new();
        change(&coins, amount, &mut memo)
    }
}

impl CoinChange for Solution {
    fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        Self::coin_change_tabulation(coins, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p322_test() {
        run_tests(&Solution::coin_change_tabulation);
        run_tests(&Solution::coin_change_memo);
    }

    fn run_tests<F>(sol: &F)
    where
        F: Fn(Vec<i32>, i32) -> i32,
    {
        assert_eq!(sol(vec![2, 5, 10, 1], 27), 4);
        assert_eq!(sol(vec![186, 419, 83, 408], 6249), 20);
        assert_eq!(sol(vec![1, 2, 5], 11), 3);
        assert_eq!(sol(vec![2], 3), -1);
        assert_eq!(sol(vec![1], 0), 0);
    }
}
