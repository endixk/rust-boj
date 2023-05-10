// BOJ 17626 [Four Squares]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let n: usize = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut d = vec![4; n + 1]; d[0] = 0;
    for i in 0..n {
        for j in 1..=n {
            if i + j*j > n { break; }
            d[i + j*j] = d[i + j*j].min(d[i] + 1);
        }
    }
    println!("{}", d[n]);
}
