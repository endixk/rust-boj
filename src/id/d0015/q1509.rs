// BOJ 1509 [Palindrome Partitioning]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let s = s.as_bytes();
    let n = s.len();

    let mut ispal = vec![vec![false; n]; n];
    for i in 0..n { ispal[i][i] = true; }
    for i in 0..n-1 { ispal[i][i+1] = s[i] == s[i+1]; }
    for i in 2..n {
        for j in 0..n-i {
            ispal[j][j+i] = ispal[j+1][j+i-1] && s[j] == s[j+i];
        }
    }

    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 1..n {
        dp[i] = dp[i-1] + 1;
        for j in 0..i {
            if ispal[j][i] {
                dp[i] = dp[i].min(if j == 0 { 1 } else { dp[j-1] + 1 });
            }
        }
    }
    println!("{}", dp[n-1]);
}
