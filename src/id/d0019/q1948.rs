// BOJ 1948 [Critical Path]
// Supported by GitHub Copilot

use std::io::{self, Read};
use std::collections::VecDeque;

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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n];
    let mut ind = vec![0; n];
    for _ in 0..next(&mut it) {
        let (u, v, w) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, next::<u32>(&mut it));
        adj[u].push((v, w, false));
        ind[v] += 1;
    }
    let (s, e) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);

    // topological sort
    let mut dist = vec![0; n];
    let mut trk = vec![vec![]; n];
    let mut q = VecDeque::new();
    q.push_back(s);
    while let Some(u) = q.pop_front() {
        for &(v, w, _) in &adj[u] {
            if dist[v] < dist[u] + w {
                dist[v] = dist[u] + w;
                trk[v].clear();
                trk[v].push(u);
            } else if dist[v] == dist[u] + w {
                trk[v].push(u);
            }
            ind[v] -= 1;
            if ind[v] == 0 {
                q.push_back(v);
            }
        }
    }

    let mut vis = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(e);
    while let Some(u) = q.pop_front() {
        for &v in &trk[u] {
            let i = adj[v].iter().position(|&(_u, _, _)| _u == u).unwrap();
            adj[v][i].2 = true;
            if !vis[v] {
                vis[v] = true;
                q.push_back(v);
            }
        }
    }

    println!("{} {}", dist[e], adj.iter().map(|v| v.iter().filter(|&&(_, _, b)| b).count()).sum::<usize>());
}
