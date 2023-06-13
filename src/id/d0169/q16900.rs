// BOJ 16900 [Naming]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

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
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (s, n) = (next::<String>(&mut it), next::<usize>(&mut it));

    let p = pi(&s);
    println!("{}", s.len() + (s.len() - p[s.len()-1]) * (n-1));
}
