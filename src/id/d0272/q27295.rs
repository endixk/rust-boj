// BOJ 27295 [Linear Regression 1]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, b) = (next::<i64>(&mut it), next::<i64>(&mut it));
    let (mut x, mut y) = (0, 0);
    for _ in 0..n {
        x += next::<i64>(&mut it);
        y += next::<i64>(&mut it);
    }

    if x == 0 { writeln!(so, "EZPZ").ok(); }
    else {
        let (p, q) = (y - n * b, x);
        if p == 0 { writeln!(so, "0").ok(); return; }
        let sign = (p > 0) ^ (q > 0);
        if sign { write!(so, "-").ok(); }

        let (p, q) = (p.abs(), q.abs());
        let g = gcd(p, q);
        if q / g > 1 {
            writeln!(so, "{}/{}", p / g, q / g).ok();
        } else {
            writeln!(so, "{}", p / g).ok();
        }
    }
}
