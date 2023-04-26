// BOJ 1929 [Prime Numbers]
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

fn sieve(n: usize) -> Vec<bool> {
    let mut v = vec![true; n+1];
    v[1] = false;
    (2..=(n as f64).sqrt() as usize).for_each(|i| if v[i] {
        (i*i..=n).step_by(i).for_each(|j| v[j] = false);
    });
    v
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (m, n) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let p = sieve(n);
    (m..=n).for_each(|i| if p[i] { write!(so, "{} ", i).ok(); });
    writeln!(so).ok();
}
