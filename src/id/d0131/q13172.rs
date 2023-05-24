// BOJ 13172 [Σ]
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
    return if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = xgcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}
fn mmi(a: i64, m: i64) -> i64 {
    let (_, x, _) = xgcd(a, m);
    (x + m) % m
}

const MOD: i64 = 1_000_000_007;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (mut n, mut d) = (0, 1);
    (0..next(&mut it)).for_each(|_| {
        let (b, a) = (next::<i64>(&mut it), next::<i64>(&mut it));
        n = (n * b + a * d) % MOD;
        d = d * b % MOD;
    });
    writeln!(so, "{}", n * mmi(d, MOD) % MOD).ok();
}
