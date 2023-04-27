// BOJ 2231 [Digit Generator]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();

    (if n>55 {n-55} else {0}..n).for_each(|i| {
        let (mut gen, mut t) = (i, i);
        while t > 0 {
            gen += t % 10;
            t /= 10;
        }
        if gen == n {
            println!("{}", i);
            std::process::exit(0);
        }
    });
    println!("0");
}
