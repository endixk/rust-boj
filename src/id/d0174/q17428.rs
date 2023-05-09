// BOJ 17428 [Kth Parenthesis String]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, mut k) = (next::<usize>(&mut it) / 2, next::<u64>(&mut it) + 1);
    let mut dp = vec![vec![0u64; n+1]; n+1];
    dp[0][0] = 1;
    for i in 0..=n { for j in 0..=n {
        if i+1 <=n { dp[i+1][j] += dp[i][j]; }
        if j+1 <=n && i > j { dp[i][j+1] += dp[i][j]; }
    }}

    if dp[n][n] < k { writeln!(so, "-1").unwrap(); return; }
    k = dp[n][n] - k + 1;
    let mut ans = String::new();
    let (mut i, mut j) = (n, n);
    while i > 0 || j > 0 {
        if i > 0 && dp[i-1][j] >= k {
            ans.push(')');
            i -= 1;
        } else {
            ans.push('(');
            k -= dp[i-1][j];
            j -= 1;
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
