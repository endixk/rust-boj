// BOJ 2482 [Color Wheel]
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

const MOD: i32 = 1_000_000_003;
fn go(dp: &mut Vec<Vec<i32>>, n: usize, k: usize) -> i32 {
    if k == 0 { return 1; }
    if k == 1 { return n as i32; }
    if n < 2 * k - 1 { return 0; }
    if dp[n][k] != -1 { return dp[n][k]; }
    dp[n][k] = go(dp, n - 1, k) + go(dp, n - 2, k - 1);
    dp[n][k] %= MOD;
    dp[n][k]
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut dp = vec![vec![-1; k + 1]; n + 1];
    writeln!(so, "{}", (go(&mut dp, n-1, k) + go(&mut dp, n-3, k-1)) % MOD)?;

    Ok(())
}
