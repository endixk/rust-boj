// BOJ 24428 [Matrix Path 5]
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

    let mut dp = vec![vec![0; n]; n];
    dp[0][0] = mat[0][0];
    for i in 1..n { dp[i][0] = dp[i-1][0] + mat[i][0]; }
    for j in 1..n { dp[0][j] = dp[0][j-1] + mat[0][j]; }
    for i in 1..n { for j in 1..n {
        dp[i][j] = mat[i][j] + dp[i-1][j].max(dp[i][j-1]);
    }}

    for _ in 0..3 {
        let mut tp = vec![vec![0; n]; n];
        for i in 0..n { for j in 0..n {
            if i > 0 {
                if p[i][j] && dp[i-1][j] > 0 {
                    tp[i][j] = tp[i][j].max(dp[i-1][j] + mat[i][j]);
                } else if tp[i-1][j] > 0 {
                    tp[i][j] = tp[i][j].max(tp[i-1][j] + mat[i][j]);
                }
            }
            if j > 0 {
                if p[i][j] && dp[i][j-1] > 0 {
                    tp[i][j] = tp[i][j].max(dp[i][j-1] + mat[i][j]);
                } else if tp[i][j-1] > 0 {
                    tp[i][j] = tp[i][j].max(tp[i][j-1] + mat[i][j]);
                }
            }
        }}
        dp = tp;
    }

    writeln!(so, "{}", if dp[n-1][n-1] > 0 { dp[n-1][n-1] } else { -1 })?;

    Ok(())
}
