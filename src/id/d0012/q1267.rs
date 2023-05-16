// BOJ 1267 [Phone Bill]
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

    let v = (0..next(&mut it)).fold((0, 0), |v, _| {
        let t = next::<i32>(&mut it);
        (v.0 + (t / 30 + 1) * 10, v.1 + (t / 60 + 1) * 15)
    });
    match v.0 - v.1 {
        x if x < 0 => writeln!(so, "Y {}", v.0).ok(),
        x if x > 0 => writeln!(so, "M {}", v.1).ok(),
        _ => writeln!(so, "Y M {}", v.0).ok()
    };
}
