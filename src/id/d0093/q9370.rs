// BOJ 9370 [Destination Unknown]
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

use std::collections::BinaryHeap;
use std::cmp::Reverse;

const INF: u32 = 0x3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(u32, u32)>>, par: &mut Vec<Vec<u32>>, dist: &mut [u32], src: u32) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src as usize] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u as usize] < d { continue; }
        for &(v, w) in &adj[u as usize] {
            if dist[v as usize] == dist[u as usize] + w {
                par[v as usize].push(u);
            }
            if dist[v as usize] > dist[u as usize] + w {
                par[v as usize].clear();
                par[v as usize].push(u);
                dist[v as usize] = dist[u as usize] + w;
                pq.push((Reverse(dist[v as usize]), v));
            }
        }
    }
}

fn dfs(dt: &Vec<Vec<u32>>, vis: &mut [bool], u: u32) {
    vis[u as usize] = true;
    for &v in &dt[u as usize] {
        if !vis[v as usize] { dfs(dt, vis, v); }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next::<usize>(&mut it) {
        let (n, m, t) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        let (s, g, h) = (next::<u32>(&mut it), next::<u32>(&mut it), next::<u32>(&mut it));
        let mut adj = vec![vec![]; n+1];
        for _ in 0..m {
            let (u, v, w) = (next::<u32>(&mut it), next::<u32>(&mut it), next::<u32>(&mut it));
            adj[u as usize].push((v, w));
            adj[v as usize].push((u, w));
        }

        let mut par = vec![vec![]; n+1];
        let mut dist = vec![INF; n+1];
        dijkstra(&adj, &mut par, &mut dist, s);

        let mut dt = vec![vec![]; n+1];
        for u in 1..=n { for &p in &par[u] { dt[p as usize].push(u as u32); } }

        let src = if dist[g as usize] < dist[h as usize] { h } else { g };
        let mut vis = vec![false; n+1];
        dfs(&dt, &mut vis, src);

        let mut d = (0..t).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        d.sort_unstable();
        d.into_iter().filter(|&x| vis[x]).for_each(|x| write!(so, "{} ", x).unwrap());
        writeln!(so)?;
    }

    Ok(())
}
