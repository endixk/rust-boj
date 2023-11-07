// BOJ 30413 [Counting Sheep]
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
fn pow(a: i64, p: i64, m: i64) -> i64 {
    if p == 0 { return 1; }
    let mut ret = pow(a, p / 2, m);
    ret = (ret * ret) % m;
    if p % 2 == 1 { ret = (ret * a) % m; }
    ret
}

const MOD: i64 = 1_000_000_007;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (a, b) = (next::<i64>(&mut it), next::<i64>(&mut it));
    if a == 1 {
        writeln!(so, "{}", b % MOD)?;
    } else {
        writeln!(so, "{}", (((pow(a, b, MOD) - 1) * mmi(a - 1, MOD) % MOD) + MOD) % MOD)?;
    }

    Ok(())
}
