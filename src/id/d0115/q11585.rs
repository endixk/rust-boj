// BOJ 11585 [Dinner Menu]
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

fn kmp(s: &String, t: &String) -> u32 {
    let p = pi(t);
    let (s, t) = (s.as_bytes(), t.as_bytes());
    let (n, m) = (s.len(), t.len());

    let (mut occ, mut j) = (0, 0);
    for i in 0..n {
        while j > 0 && s[i] != t[j] { j = p[j-1]; }
        if s[i] == t[j] {
            if j == m-1 { occ += 1; j = p[j]; }
            else { j += 1; }
        }
    }
    occ
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a }
    else { gcd(b, a % b) }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let s = (0..n).map(|_| next::<char>(&mut it)).collect::<String>();
    let t = (0..n).map(|_| next::<char>(&mut it)).collect::<String>();

    let s = s.clone() + &s[..n-1];
    let x = kmp(&s, &t);
    println!("{}/{}", x / gcd(x, n as u32), n as u32 / gcd(x, n as u32));
}
