// BOJ 2447 [Printing Stars - 10]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn get(mut i: usize, mut j: usize) -> bool {
    while i > 0 {
        if i % 3 == 1 && j % 3 == 1 {
            return false;
        }
        i /= 3; j /= 3;
    }
    true
}
pub fn main() {
    let n: usize = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut so = io::BufWriter::new(io::stdout().lock());
    for i in 0..n {
        for j in 0..n {
            write!(so, "{}", if get(i, j) { '*' } else { ' ' }).ok();
        }
        writeln!(so).ok();
    }
}
