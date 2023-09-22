// BOJ 24430 [Matrix Path 7]
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
    let mut mat = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        mat[i][j] = next::<i64>(&mut it);
    }}
    let mut p = vec![vec![false; n]; n];
    for _ in 0..next(&mut it) {
        p[next::<usize>(&mut it)-1][next::<usize>(&mut it)-1] = true;
    }

    let mut dp = vec![vec![(0, 0); n]; n];
    dp[0][0] = (mat[0][0], 0);
    for i in 1..n {
        dp[0][i] = (dp[0][i-1].0 + mat[0][i], dp[0][i-1].1 + if p[0][i] { 1 } else { 0 });
        dp[i][0] = (dp[i-1][0].0 + mat[i][0], dp[i-1][0].1 + if p[i][0] { 1 } else { 0 });
    }
    for i in 1..n { for j in 1..n {
        dp[i][j] = if dp[i-1][j].0 < dp[i][j-1].0 || (dp[i-1][j].0 == dp[i][j-1].0 && dp[i-1][j].1 < dp[i][j-1].1){
            (dp[i][j-1].0 + mat[i][j], dp[i][j-1].1 + if p[i][j] { 1 } else { 0 })
        } else {
            (dp[i-1][j].0 + mat[i][j], dp[i-1][j].1 + if p[i][j] { 1 } else { 0 })
        }
    }}
    writeln!(so, "{} {}", dp[n-1][n-1].0, dp[n-1][n-1].1)?;

    Ok(())
}
