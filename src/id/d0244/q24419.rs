// BOJ 24419 [Matrix Path 2]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut dp = vec![vec![1; n+1]; n+1];
    for i in 1..=n { for j in 1..=n {
        dp[i][j] = (dp[i-1][j] + dp[i][j-1]) % 1000000007;
    }}
    writeln!(so, "{} {}", dp[n][n], n*n)?;

    Ok(())
}
