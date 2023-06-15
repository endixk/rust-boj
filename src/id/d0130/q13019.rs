// BOJ 13019 [A into B]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let t = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut sv = s.chars().map(|c| c as u8).collect::<Vec<_>>();
    let mut tv = t.chars().map(|c| c as u8).collect::<Vec<_>>();
    sv.sort_unstable();
    tv.sort_unstable();
    if sv != tv {
        println!("-1");
        return;
    }

    let (s, t) = (s.as_bytes(), t.as_bytes());
    let mut ans = 0;
    let (mut i, mut j) = (s.len(), t.len());
    while i > 0 {
        if s[i-1] == t[j-1] {
            i -= 1;
            j -= 1;
        } else {
            ans += 1;
            i -= 1;
        }
    }
    println!("{}", ans);
}
