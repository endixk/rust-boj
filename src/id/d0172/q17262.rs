// BOJ 17262 [Fandom]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let (mut s, mut e) = (111111, 0);
    io::stdin().lock().lines().skip(1).for_each(|buf| {
        let buf = buf.unwrap();
        let mut it = buf.split_ascii_whitespace();
        e = e.max(it.next().unwrap().parse().unwrap());
        s = s.min(it.next().unwrap().parse().unwrap());
    });
    println!("{}", if s > e { 0 } else { e - s });
}
