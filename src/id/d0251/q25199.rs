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

const MOD: i64 = 1_000_000_007;
fn go(dp: &mut Vec<Vec<i64>>, i: usize, j: usize, x: usize, k: usize) -> i64 {
    if i == 0 { return 1; }
    if j == 0 { return 1; }
    if dp[i][j] != -1 { return dp[i][j]; }
    let c = if i < x { k } else { k - 1 };
    let ret = (go(dp, i, j-1, x, k) + go(dp, i-1, j, x, k) - if j > c { go(dp, i-1, j-c-1, x, k) } else { 0 } + MOD) % MOD;
    dp[i][j] = ret;
    ret
}
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
    writeln!(so, "{}", (go(&mut dp, m, n, x, k) - go(&mut dp, m, n-1, x, k) + MOD) % MOD).unwrap();
}
