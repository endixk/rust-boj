// BOJ 6064 [Cain Calendar]
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

fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x, y) = xgcd(b, a % b);
    (g, y, x - (a / b) * y)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (m, n, x, y) = (
            next::<i64>(&mut it), next::<i64>(&mut it),
            next::<i64>(&mut it), next::<i64>(&mut it));

        let (g, k, _) = xgcd(m, n);
        if (y - x) % g != 0 { writeln!(so, "-1").unwrap(); }
        else {
            let k = k * (y - x) / g;
            let mut k = (m * k + x) % (m * n / g);
            while k <= 0 { k += m * n / g; }
            writeln!(so, "{}", k).unwrap();
        }
    }
}
