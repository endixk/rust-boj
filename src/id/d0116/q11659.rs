// BOJ 11659 [Interval Sum 4]
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
    let (n, m) = (next(&mut it), next(&mut it));

    let mut a = vec![0; n+1];
    for i in 1..=n {
        let x: i32 = next(&mut it);
        a[i] = a[i-1] + x;
    }
    for _ in 0..m {
        let (i, j): (usize, usize) = (next(&mut it), next(&mut it));
        writeln!(so, "{}", a[j] - a[i-1]).unwrap();
    }
}
