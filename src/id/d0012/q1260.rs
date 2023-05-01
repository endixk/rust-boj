// BOJ 1260 [DFS and BFS]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn dfs(u: usize, adj: &Vec<Vec<usize>>, so: &mut io::BufWriter<io::StdoutLock>) {
    let mut vis = vec![false; adj.len()];
    let mut st = vec![u];
    while let Some(u) = st.pop() {
        if vis[u] { continue }
        vis[u] = true;
        write!(so, "{} ", u).ok();
        for &v in adj[u].iter().rev() {
            if !vis[v] { st.push(v) }
        }
    }
}
fn bfs(u: usize, adj: &Vec<Vec<usize>>, so: &mut io::BufWriter<io::StdoutLock>) {
    let mut vis = vec![false; adj.len()];
    let mut q = VecDeque::new();
    q.push_back(u);
    while let Some(u) = q.pop_front() {
        if vis[u] { continue }
        vis[u] = true;
        write!(so, "{} ", u).ok();
        for &v in &adj[u] {
            if !vis[v] { q.push_back(v) }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m, v) = (next(&mut it), next(&mut it), next(&mut it));
    let mut adj = vec![vec![]; n + 1];
    (0..m).for_each(|_| {
        let (a, b) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[a].push(b);
        adj[b].push(a);
    });

    for i in 1..=n { adj[i].sort(); }
    dfs(v, &adj, &mut so);
    writeln!(so).ok();
    bfs(v, &adj, &mut so);
    writeln!(so).ok();
}
