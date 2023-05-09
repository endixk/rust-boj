// BOJ 11047 [Coin 0]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, mut k) = (next::<usize>(&mut it), next::<u32>(&mut it));
    let mut ans = 0;
    (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>()
        .iter().rev().for_each(|&x| { ans += k / x; k %= x; });
    println!("{}", ans);
}
