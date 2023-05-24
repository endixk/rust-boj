// BOJ 16953 [A → B]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut it = s.split_whitespace();
    let (a, mut b) = (it.next().unwrap().parse::<u32>().unwrap(), it.next().unwrap().parse::<u32>().unwrap());

    let mut c = 1;
    while a < b {
        if b % 2 == 0 { b /= 2 }
        else if b % 10 == 1 { b /= 10 }
        else { break }
        c += 1;
    }
    if a == b { println!("{}", c) }
    else { println!("-1") }
}
