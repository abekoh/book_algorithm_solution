#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn frog(stages: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; stages.len()];
        for i in 0..stages.len() {
            match i {
                0 => dp[i] = 0,
                1 => dp[i] = (stages[i] - stages[i - 1]).abs(),
                _ => {
                    dp[i] = std::cmp::min(
                        dp[i - 1] + (stages[i] - stages[i - 1]).abs(),
                        dp[i - 2] + (stages[i] - stages[i - 2]).abs(),
                    )
                }
            }
        }
        dp[stages.len() - 1]
    }
}

#[cfg(test)]
mod ch05_02_dp_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::frog(vec![2, 9, 4, 5, 1, 6, 10]), 8);
    }
}

fn main() {
    println!("leetcode template");
}
