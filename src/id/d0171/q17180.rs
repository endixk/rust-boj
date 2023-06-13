// BOJ 17180 [String Comparison]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn dist(p: u8, q: u8) -> u32 {
    (if p > q { p - q } else { q - p }) as u32
}

pub fn main() {
    let mut it = io::stdin().lock().lines().skip(1);
    let (s, t) = (it.next().unwrap().unwrap(), it.next().unwrap().unwrap());
    let (s, t) = (s.as_bytes(), t.as_bytes());

    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0; m]; n];
    dp[0][0] = dist(s[0], t[0]);
    for i in 1..n { dp[i][0] = dp[i - 1][0] + dist(s[i], t[0]); }
    for j in 1..m { dp[0][j] = dp[0][j - 1] + dist(s[0], t[j]); }
    for i in 1..n { for j in 1..m {
        dp[i][j] = dist(s[i], t[j]);
        dp[i][j] += dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1]);
    }}

    println!("{}", dp[n - 1][m - 1]);
}
