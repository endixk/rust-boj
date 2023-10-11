// BOJ 15681 [Tree and Queries]
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

fn dfs(adj: &Vec<Vec<usize>>, dp: &mut Vec<i32>, cur: usize, par: usize) -> i32 {
    dp[cur] = 1;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        dp[cur] += dfs(adj, dp, nxt, cur);
    }
    dp[cur]
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, r, q) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut dp = vec![0; n+1];
    dfs(&adj, &mut dp, r, 0);
    for _ in 0..q {
        writeln!(so, "{}", dp[next::<usize>(&mut it)])?;
    }

    Ok(())
}
