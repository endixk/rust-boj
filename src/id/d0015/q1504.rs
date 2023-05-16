// BOJ 1504 [Specific Shortest Path]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

const INF: usize = 0x3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for (v, w) in &adj[u] {
            if dist[*v] > dist[u] + *w {
                dist[*v] = dist[u] + *w;
                pq.push((Reverse(dist[*v]), *v));
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n + 1];
    for _ in 0..next(&mut it) {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut ds = vec![INF; n + 1];
    dijkstra(&adj, &mut ds, 1);
    let mut de = vec![INF; n + 1];
    dijkstra(&adj, &mut de, n);
    let mut du = vec![INF; n + 1];
    dijkstra(&adj, &mut du, u);

    let (uv, vu) = (ds[u] + du[v] + de[v], ds[v] + du[v] + de[u]);
    let ans = if uv < vu { uv } else { vu };
    writeln!(so, "{}", if ans >= INF { -1 } else { ans as i32 }).ok();
}
