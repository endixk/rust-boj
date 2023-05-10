// BOJ 11727 [2 by N Tiling 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let mut fib = vec![1, 1];
    for i in 1..io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap() {
        fib.push(((fib[i-1] << 1) + fib[i]) % 10_007);
    }
    println!("{}", fib.last().unwrap());
}
