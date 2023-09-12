// BOJ 2572 [Board Game]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<char>(&mut it)).collect::<Vec<_>>();
    let (m, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; m+1];
    for _ in 0..k {
        let (u, v, c) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<char>(&mut it));
        adj[u].push((v, c));
        adj[v].push((u, c));
    }

    let mut dp = vec![vec![-1; m+1]; n+1];
    dp[0][1] = 0;
    for i in 1..=n {
        for j in 1..=m {
            for &(v, c) in &adj[j] {
                if dp[i-1][v] >= 0 {
                    dp[i][j] = dp[i][j].max(dp[i-1][v] + if c == a[i-1] { 10 } else { 0 });
                }
            }
        }
    }
    let ans = *dp[n].iter().max().unwrap();
    writeln!(so, "{}", if ans < 0 { 0 } else { ans } ).unwrap();
}
