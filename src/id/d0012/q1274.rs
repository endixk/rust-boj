// BOJ 1274 [Coffee Shop 1]
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

fn go(a: f64, b: f64, x: i32, y: i32, s: i32) -> (f64, f64, f64) {
    let (x, y, s) = (x as f64, y as f64, s as f64);

    // drink A
    let ca = a * (x - s) / 1000.0;
    // pour B
    let pb = b * s / 1000.0;
    let ca = ca + pb;
    // fill B
    let cb = b * (y - s) / 1000.0;

    (a, ca / x * 1000.0, cb / y * 1000.0)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (mut a, mut b, x, y, s) = (
        next::<f64>(&mut it), next::<f64>(&mut it),
        next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it),
    );

    if x < s || y < s {
        writeln!(so, "gg").ok();
        return;
    }

    let (mut m, mut n) = (a, 0);
    for _ in 0..10000 {
        if n > 50 { break }
        let (c, na, nb) = go(a, b, x, y, s);
        if (m - c).abs() < 1e-9 { n += 1 }
        else if c > m { m = c; n += 1 }
        (a, b) = (na, nb);
    }
    if n > 50 { writeln!(so, "gg").ok(); }
    else { writeln!(so, "{:X}", n).ok(); }
}
