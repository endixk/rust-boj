// BOJ 1787 [Periods of Words]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
fn pi(s: &String) -> Vec<usize> {
    let (s, l) = (s.as_bytes(), s.len());
    let mut p = vec![0; l];
    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }
    p
}

const INF: usize = 0x3f3f3f3f;
fn solve(dp: &mut [usize], pi: &[usize], i: usize) -> usize {
    if i == 0 || pi[i] == 0 { return INF; }
    if dp[i] != INF { return dp[i]; }
    dp[i] = pi[i].min(solve(dp, pi, pi[i]-1));
    dp[i]
}
pub fn main() {
    let s = io::stdin().lock().lines().skip(1).next().unwrap().unwrap();
    let pi = pi(&s);
    let mut dp = vec![INF; s.len()];

    let mut ans = 0;
    for i in 1..s.len() {
        let x = solve(&mut dp, &pi, i);
        if x < INF { ans += i+1-x; }
    }
    println!("{}", ans);
}
