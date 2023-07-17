// BOJ 5875 [Typo]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();

    let mut x = 0;
    let mut v = Vec::with_capacity(s.len());
    for c in s.chars() {
        if c == '(' { x += 1; }
        else { x -= 1; }
        v.push(x);
    }

    let mut nv = vec![999999; s.len()];
    nv[s.len()-1] = v[s.len()-1];
    for (i, &x) in v.iter().enumerate().rev().skip(1) {
        nv[i] = nv[i+1].min(x);
    }

    let mut ans = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' if nv[i] == 2 => { ans += 1; },
            ')' if nv[i] == -2 => { ans += 1; },
            _ => continue,
        }
        if v[i] < 0 { break; }
    }
    println!("{}", ans);
}
