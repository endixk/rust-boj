// BOJ 11725 [Find the Parent Node]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

const MAX: usize = 100_001;

fn dfs(adj: &Vec<Vec<usize>>, parent: &mut Vec<usize>) {
    let mut stack = Vec::new();
    stack.push(1);
    parent[0] = 1;
    parent[1] = 1;
    while let Some(u) = stack.pop() {
        for &v in &adj[u] {
            if parent[v] == 0 {
                parent[v] = u;
                stack.push(v);
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = it.by_ref().next().unwrap().parse().unwrap();
    let mut adj = vec![Vec::new(); n+1];
    let mut parent = vec![0; n+1];
    for _ in 1..n {
        let u: usize = it.by_ref().next().unwrap().parse().unwrap();
        let v: usize = it.by_ref().next().unwrap().parse().unwrap();
        adj[u].push(v);
        adj[v].push(u);
    }

    dfs(&adj, &mut parent);
    for i in 2..=n {
        writeln!(so, "{}", parent[i]).unwrap();
    }
}
