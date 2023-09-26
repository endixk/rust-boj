// BOJ 30013 [Dolbear's Law]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = io::stdin().lock().lines().skip(1).next().unwrap()?;
    let s = s.chars().collect::<Vec<char>>();

    let mut ans = s.len();
    for f in 1..s.len() {
        let mut c = 0;
        for i in 0..s.len() {
            if s[i] == '#' && (i < f || s[i-f] == '.') { c += 1; }
        }
        ans = ans.min(c);
    }
    println!("{}", ans);

    Ok(())
}
