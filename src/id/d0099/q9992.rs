// BOJ 9992 [Secret Code]
use std::io::BufRead;
const M: u32 = 2014;
fn go(s: &String, dp: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if dp[i][j] != M { return dp[i][j]; }
    if i == j { return 1; }
    let mut ret = 1;
    for k in 1..=(j-i)/2 {
        let (h, t) = (&s[i..i+k], &s[j-k+1..=j]);
        if s[i+k..=j].starts_with(h) { ret += go(s, dp, i+k, j); }
        if s[i+k..=j].ends_with(h) { ret += go(s, dp, i+k, j); }
        if s[i..=j-k].starts_with(t) { ret += go(s, dp, i, j-k); }
        if s[i..=j-k].ends_with(t) { ret += go(s, dp, i, j-k); }
    }
    dp[i][j] = ret % M;
    ret
}
pub fn main() {
    let s = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let n = s.len();
    println!("{}", (go(&s, &mut vec![vec![M; n]; n], 0, n-1) + M - 1) % M);
}