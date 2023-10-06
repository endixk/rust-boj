// BOJ 1976 [Journey]
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

fn dfs(adj: &Vec<Vec<usize>>, vis: &mut [bool], u: usize) {
    vis[u] = true;
    for &v in &adj[u] {
        if !vis[v] { dfs(adj, vis, v); }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n+1];
    for i in 1..=n { for j in 1..=n {
        if next::<u8>(&mut it) == 1 { adj[i].push(j); }
    }}

    let mut vis = vec![false; n+1];
    let a = (0..m).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    dfs(&adj, &mut vis, a[0]);
    for u in a {
        if !vis[u] {
            writeln!(so, "NO")?;
            return Ok(());
        }
    }
    writeln!(so, "YES")?;

    Ok(())
}
