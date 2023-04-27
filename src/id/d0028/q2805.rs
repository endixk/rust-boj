// BOJ 2805 [EKO]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<u64>(&mut it));
    let v = (0..n).map(|_| next::<u64>(&mut it)).collect::<Vec<_>>();

    // parametric search
    let (mut lo, mut hi) = (0u64, *v.iter().max().unwrap());
    while lo + 1 < hi {
        let mid = (lo + hi) >> 1;
        if v.iter().map(|&x| if x > mid { x - mid } else { 0 }).sum::<u64>() >= m { lo = mid } else { hi = mid }
    }

    writeln!(so, "{}", lo).unwrap();
}
