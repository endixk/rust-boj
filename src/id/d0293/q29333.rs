// BOJ 29333 [One Walk]
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

use std::collections::VecDeque;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, s, e) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it),
    );

    let mut edges = Vec::new();
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
        edges.push((u, v));
    }

    // find shortest path from s to e
    let mut vis = vec![false; n+1];
    let mut par = vec![0; n+1];
    vis[s] = true; par[s] = 0;
    let mut q = VecDeque::new();
    q.push_back(s);
    while let Some(u) = q.pop_front() {
        if u == e { break; }
        for &v in &adj[u] {
            if !vis[v] {
                vis[v] = true;
                par[v] = u;
                q.push_back(v);
            }
        }
    }
    if !vis[e] { writeln!(so, "-1").ok(); return; }

    // backtrack path
    let mut deg = vec![n+1; n+1];
    let (mut u, mut d) = (e, 1);
    while u != s {
        deg[u] = d;
        u = par[u];
        d += 1;
    }
    deg[s] = d;

    for (u, v) in edges {
        if deg[u] > deg[v] {
            write!(so, "0 ").ok();
        } else {
            write!(so, "1 ").ok();
        }
    }
    writeln!(so).ok();
}
