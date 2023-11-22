// BOJ 30643 [Wooden Bridge]
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

    const MOD: i64 = 1_000_000_007;
    let (w, l) = (next::<usize>(&mut it), next::<usize>(&mut it));
    if l == 2 { writeln!(so, "1")?; return Ok(()); }

    let mut dp = vec![vec![vec![0; 4]; l-1]; w];
    for j in 0..l-1 {
        let b = if j == 0 { 1 } else if j == l-2 { 2 } else { 0 };
        dp[0][j][b] = 1;
    }
    for i in 1..w {
        for j in 0..l-1 {
            let b = if j == 0 { 1 } else if j == l-2 { 2 } else { 0 };
            if j > 0 {
                for k in 0..4 {
                    dp[i][j][b|k] += dp[i-1][j-1][k];
                    dp[i][j][b|k] %= MOD;
                }
            }
            for k in 0..4 {
                dp[i][j][b|k] += dp[i-1][j][k];
                dp[i][j][b|k] %= MOD;
            }
            if j < l-2 {
                for k in 0..4 {
                    dp[i][j][b|k] += dp[i-1][j+1][k];
                    dp[i][j][b|k] %= MOD;
                }
            }
        }
    }

    writeln!(so, "{}", (0..l-1).map(|j| dp[w-1][j][3]).sum::<i64>() % MOD)?;

    Ok(())
}
