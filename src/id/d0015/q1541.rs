// BOJ 1541 [Parenthesis Lost]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let m = s.chars().filter(|&c| c == '+' || c == '-')
        .position(|c| c == '-');
    let v = s.split(|c| c == '+' || c == '-')
        .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

    if let Some(i) = m {
        println!("{}", v[..=i].iter().sum::<i32>() - v[i+1..].iter().sum::<i32>());
    } else {
        println!("{}", v.iter().sum::<i32>());
    }
}
