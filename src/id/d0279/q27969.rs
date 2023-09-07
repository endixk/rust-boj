// BOJ 27969 [I LOVE JavaScript]
// Supported by GitHub Copilot

use std::io::BufRead;
pub fn main() {
    let s = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut ans = 0;
    for w in s.split_ascii_whitespace() {
        let w = w.as_bytes();
        match w[0] {
            b'[' | b'1'..=b'9' => ans += 8,
            b']' => (),
            _ => ans += w.len() + 12,
        }
    }
    println!("{}", ans);
}
