// BOJ 9251 [LCS]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let t = read(&mut si);
    let mut dp = [[0u16; 1001]; 1001];
    for (i, x) in s.chars().enumerate() {
        for (j, y) in t.chars().enumerate() {
            if x == y {
                dp[i+1][j+1] = dp[i][j] + 1;
            } else {
                dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }

    writeln!(so, "{}", dp[s.len()][t.len()]).unwrap();
}
