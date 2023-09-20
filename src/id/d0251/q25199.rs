// BOJ 25199 [Gambler]
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

const MOD: i32 = 1_000_000_007;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();

    let (mut x, mut k) = (0, 0);
    let (mut xi, mut ki) = (0, 0);
    for v in a {
        if xi == v { ki += 1; }
        else {
            if ki > k { x = xi; k = ki; }
            else if ki == k && xi > x { x = xi; }
            xi = v; ki = 1;
        }
    }
    if ki > k { x = xi; k = ki; }
    else if ki ==k && xi > x { x = xi; }

    let mut dp = vec![vec![-1; n + 1]; m + 1];
    for j in 0..=n { dp[0][j] = 1; }
    for i in 1..=m { dp[i][0] = 1; }
    for i in 1..=m {
        let c = if i < x { k } else { k - 1 };
        for j in 1..=n {
            dp[i][j] = ((dp[i][j-1] + dp[i-1][j]) % MOD - if j > c { dp[i-1][j-c-1] } else { 0 } + MOD) % MOD;
        }
    }
    writeln!(so, "{}", (dp[m][n] - dp[m][n-1] + MOD) % MOD).ok();
}
