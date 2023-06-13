// BOJ 12904 [A and B]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let mut it = io::stdin().lock().lines();
    let (a, mut b) = (
        it.next().unwrap().unwrap(),
        it.next().unwrap().unwrap(),
    );

    while b.len() > a.len() {
        if b.ends_with("A") {
            b.pop();
        } else {
            b.pop();
            b = b.chars().rev().collect::<String>();
        }
    }
    println!("{}", if a == b { 1 } else { 0 });
}
