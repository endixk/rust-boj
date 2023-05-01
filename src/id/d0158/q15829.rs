// BOJ 15829 [Hashing]
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

fn hash(c: char, i: usize, r: u64, m: u64) -> u64 {
    let mut x = c as u64 - 'a' as u64 + 1;
    (0..i).for_each(|_| x = (x * r) % m);
    x
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    const R: u64 = 31;
    const M: u64 = 1234567891;

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (_, s) = (next::<usize>(&mut it), next::<String>(&mut it));

    let mut h = 0;
    for (i, c) in s.chars().enumerate() {
        h = (h + hash(c, i, R, M)) % M;
    }
    writeln!(so, "{}", h).unwrap();
}
