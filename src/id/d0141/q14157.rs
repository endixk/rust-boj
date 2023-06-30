// BOJ 14157 [Count Palindromes]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

const MOD: i32 = 10007;
pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let (s, n) = (s.as_bytes(), s.len());

    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
        for j in (0..i).rev() {
            dp[j][i] = dp[j + 1][i] + dp[j][i - 1] - dp[j + 1][i - 1];
            if s[i] == s[j] {
                dp[j][i] += dp[j + 1][i - 1] + 1;
            }
            dp[j][i] = (dp[j][i] + MOD) % MOD;
        }
    }
    println!("{}", dp[0][n - 1]);
}
