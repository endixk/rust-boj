// BOJ 13275 [Longest Palindrome Substring]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn manacher(s: &[u8], n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    let (mut r, mut p) = (0, 0);
    for i in 0..n {
        if i <= r {
            a[i] = a[2 * p - i].min(r - i);
        }
        while i > a[i] && i + a[i] + 1 < n && s[i - a[i] - 1] == s[i + a[i] + 1] {
            a[i] += 1;
        }
        if r < i + a[i] {
            r = i + a[i];
            p = i;
        }
    }
    a
}

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let s = s.as_bytes();
    let mut t = vec![0; s.len() * 2 + 1];
    for i in 0..s.len() {
        t[i * 2 + 1] = s[i];
    }

    let r = manacher(&t, t.len());
    println!("{}", r.iter().max().unwrap());
}
