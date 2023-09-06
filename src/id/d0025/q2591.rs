// BOJ 2591 [Number Cards]
// Supported by GitHub Copilot

use std::io::BufRead;
pub fn main() {
    let s = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let s = s.as_bytes();
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1; dp[1] = 1;
    if s[0] == b'0' { println!("0"); return; }
    for i in 1..s.len() {
        match s[i] {
            b'0' if s[i-1] == b'0' => { println!("0"); return; }
            b'0' if s[i-1] >= b'4' => { println!("0"); return; }
            b'0' => dp[i+1] = dp[i-1],
            b'1'..=b'4' if s[i-1] == b'0' => dp[i+1] = dp[i],
            b'1'..=b'4' if s[i-1] <= b'3' => dp[i+1] = dp[i] + dp[i-1],
            b'1'..=b'4' => dp[i+1] = dp[i],
            _ if s[i-1] == b'0' => dp[i+1] = dp[i],
            _ if s[i-1] <= b'2' => dp[i+1] = dp[i] + dp[i-1],
            _ => dp[i+1] = dp[i],
        }
    }
    println!("{}", dp[s.len()]);
}
