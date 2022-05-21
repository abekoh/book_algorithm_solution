#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn distance(s: String, t: String) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; t.len() + 1]; s.len() + 1];
        dp[0][0] = 0;
        for i in 0..(s.len() + 1) {
            for j in 0..(t.len() + 1) {
                if i > 0 && j > 0 {
                    if s.chars().nth(i - 1).unwrap() == t.chars().nth(j - 1).unwrap() {
                        dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j - 1])
                    } else {
                        dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j - 1] + 1)
                    }
                }
                if i > 0 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j] + 1)
                }
                if j > 0 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i][j - 1] + 1)
                }
            }
        }
        dp[s.len()][t.len()]
    }
}

#[cfg(test)]
mod distance_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::distance(String::from("kodansha"), String::from("danshari")),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::distance(String::from("logistic"), String::from("algorithm")),
            6
        );
    }
}

fn main() {
    println!("leetcode template");
}
