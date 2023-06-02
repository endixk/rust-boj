// BOJ 13698 [Hawk Eyes]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn go(p: u8, q: u8, r: u8, s: u8, c: char) -> (u8, u8, u8, u8) {
    match c {
        'A' => (q, p, r, s),
        'B' => (r, q, p, s),
        'C' => (s, q, r, p),
        'D' => (p, r, q, s),
        'E' => (p, s, r, q),
        'F' => (p, q, s, r),
        _ => (p, q, r, s),
    }
}

pub fn main() {
    let (mut p, mut q, mut r, mut s) = (1, 0, 0, 2);
    io::stdin().lock().lines().next().unwrap().unwrap().chars().for_each(|c| {
        (p, q, r, s) = go(p, q, r, s, c);
    });
    let v = vec![p, q, r, s];
    println!("{}", v.iter().position(|&x| x == 1).unwrap() + 1);
    println!("{}", v.iter().position(|&x| x == 2).unwrap() + 1);
}
