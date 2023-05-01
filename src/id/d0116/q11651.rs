// BOJ 11650 [Coordinate Sorting 2]
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
    let mut v = (0..next(&mut it)).map(|_| {
        let x: i32 = next(&mut it);
        let y: i32 = next(&mut it);
        (y, x)
    }).collect::<Vec<_>>();

    v.sort_unstable();
    for (y, x) in v {
        writeln!(so, "{} {}", x, y).unwrap();
    }
}
