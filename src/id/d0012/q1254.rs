// BOJ 1254 [Palindrome Generation]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let s = s.as_bytes();
    let mut ans = s.len() * 2;
    for i in s.len()/2..s.len() {
        let mut x = 0;
        while i + x < s.len() && i >= x && s[i - x] == s[i + x] { x += 1; }
        if i + x == s.len() { ans = ans.min(s.len() * 2 - x * 2 + 1); }
        x = 0;
        while i + x < s.len() && i > x && s[i - x - 1] == s[i + x] { x += 1; }
        if i + x == s.len() { ans = ans.min(s.len() * 2 - x * 2); }
    }
    println!("{}", ans);
}
