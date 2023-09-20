// BOJ 10844 [Staircase Numbers]
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

const MOD: i32 = 1_000_000_000;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut dp = vec![vec![0; 10]; n];
    for j in 1..10 { dp[0][j] = 1; }
    for i in 1..n {
        for j in 0..10 {
            if j > 0 { dp[i][j] = (dp[i][j] + dp[i-1][j-1]) % MOD; }
            if j < 9 { dp[i][j] = (dp[i][j] + dp[i-1][j+1]) % MOD; }
        }
    }

    let mut ans = 0;
    for j in 0..10 { ans = (ans + dp[n-1][j]) % MOD; }
    writeln!(so, "{}", ans).unwrap();
}
