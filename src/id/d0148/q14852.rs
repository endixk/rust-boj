// BOJ 14852 [Tiling 3]
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

const MOD: u64 = 1_000_000_007;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut dp = vec![1, 1, 1, 2];

    for _ in 1..n {
        let tp = vec![
            dp[3], dp[2] + dp[3], dp[1] + dp[3], dp[0] + dp[1] + dp[2] + dp[3] * 2
        ];
        dp = tp.iter().map(|&x| x % MOD).collect::<Vec<_>>();
    }
    writeln!(so, "{}", dp[3])?;

    Ok(())
}
