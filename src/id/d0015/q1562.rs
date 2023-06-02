// BOJ 1562 [Staircase Numbers]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut dp = vec![vec![vec![0; 1 << 10]; 10]; n];

    for d in 1..10 {
        dp[0][d][1 << d] = 1;
    }
    for i in 1..n {
        for d in 0..10 {
            for m in 0..1 << 10 {
                if d > 0 {
                    dp[i][d][m | 1 << d] += dp[i - 1][d - 1][m];
                    dp[i][d][m | 1 << d] %= 1_000_000_000;
                }
                if d < 9 {
                    dp[i][d][m | 1 << d] += dp[i - 1][d + 1][m];
                    dp[i][d][m | 1 << d] %= 1_000_000_000;
                }
            }
        }
    }

    let mut ans = 0;
    for d in 0..10 {
        ans += dp[n - 1][d][(1 << 10) - 1];
        ans %= 1_000_000_000;
    }
    println!("{}", ans);
}
