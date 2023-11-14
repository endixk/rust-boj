// BOJ 2718 [Tiling a Grid With Dominoes]
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

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut dp = [1, 1, 0, 1, 1, 1];
        for _ in 1..n {
            let tp = [
                dp[5],
                dp[3] + dp[5],
                dp[4],
                dp[1] + dp[5],
                dp[2] + dp[5],
                dp[0] + dp[1] + dp[3] + dp[4] + dp[5]
            ];
            dp = tp;
        }
        writeln!(so, "{}", dp[5])?;
    }

    Ok(())
}
