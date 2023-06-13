// BOJ 12919 [A and B 2]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

fn go(a: &String, b: &String) {
    if a.len() == b.len() {
        if a == b {
            println!("1");
            std::process::exit(0);
        } else {
            return;
        }
    }
    if b.ends_with("A") {
        go(a, &b[..b.len() - 1].to_string());
    }
    if b.starts_with("B") {
        let mut next = b[1..].to_string();
        next = next.chars().rev().collect();
        go(a, &next);
    }
}

pub fn main() {
    let mut it = io::stdin().lock().lines();
    let (a, b) = (
        it.next().unwrap().unwrap(),
        it.next().unwrap().unwrap(),
    );

    go(&a, &b);
    println!("0");
}
