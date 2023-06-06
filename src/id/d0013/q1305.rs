// BOJ 1305 [Advertisement]
// Supported by GitHub Copilot

use std::io::{stdin, BufRead};

fn pi(s: &String) -> Vec<usize> {
    let (s, l) = (s.as_bytes(), s.len());
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

pub fn main() {
    stdin().lock().lines().next().unwrap().unwrap();
    let s = stdin().lock().lines().next().unwrap().unwrap();
    let p = pi(&s);
    println!("{}", s.len() - p[s.len()-1]);
}
