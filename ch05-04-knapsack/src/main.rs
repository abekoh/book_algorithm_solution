#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn knapsack(weights: Vec<i32>, values: Vec<i32>, n: usize, max_w: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; max_w as usize + 1]; n + 1];
        for i in 0..n {
            for w in 0..=max_w {
                if w - weights[i] >= 0 {
                    dp[i + 1][w as usize] = std::cmp::max(
                        dp[i + 1][w as usize],
                        dp[i][(w - weights[i]) as usize] + values[i],
                    );
                }
                dp[i + 1][w as usize] = std::cmp::max(dp[i + 1][w as usize], dp[i][w as usize]);
            }
        }
        dp[n][max_w as usize]
    }
}

#[cfg(test)]
mod knapsack_tests {
    use super::*;

    #[test]
    fn example_1() {
        let weights = vec![2, 1, 3, 2, 1, 5];
        let values = vec![3, 2, 6, 1, 3, 85];
        let n = 6;
        assert_eq!(
            Solution::knapsack(weights.clone(), values.clone(), n, 9),
            94
        );
    }
}

fn main() {
    println!("leetcode template");
}
