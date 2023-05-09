// BOJ 1251 [Word Split]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut v = Vec::new();
    for i in 1..s.len()-1 {
        for j in i+1..s.len() {
            let mut t = s[..i].chars().rev().collect::<String>();
            t += &s[i..j].chars().rev().collect::<String>();
            t += &s[j..].chars().rev().collect::<String>();
            v.push(t);
        }
    }
    v.sort();
    println!("{}", v[0]);
}
