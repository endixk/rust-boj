// BOJ 9461 [Padovan Sequence]
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
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut ps: Vec<i64> = vec![0, 1, 1, 1, 2, 2];
    for i in 6..101 {
        ps.push(ps[i - 1] + ps[i - 5]);
    }
    for _ in 0..next(&mut it) {
        writeln!(so, "{}", ps[next::<usize>(&mut it)]).ok();
    }
}
