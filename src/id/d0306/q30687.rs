// BOJ 30687 [Pizza Stack]
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
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    const MOD: i64 = 1_000_000_007;
    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let mut p = vec![0; n];
    p[n-1] = a[n-1];
    for i in (1..n).rev() {
        p[i-1] = p[i] + a[i-1];
    }

    let mut f = 1;
    for i in 2..=p[0] { f = f * i % MOD; }
    for i in 0..n {
        for j in 2..=a[i] {
            f = f * mmi(j, MOD) % MOD;
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans += f * a[i] % MOD * mmi(p[i], MOD) % MOD;
        ans %= MOD;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
