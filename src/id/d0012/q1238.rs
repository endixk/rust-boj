// BOJ 1238 [Silver Cow Party]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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
    let (n, m, x) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; n];
    let mut rev_adj = vec![vec![]; n];
    (0..m).for_each(|_| {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u-1].push((v-1, w));
        rev_adj[v-1].push((u-1, w));
    });

    const INF: usize = 0x3f3f3f3f;
    let mut dx = vec![INF; n];
    dijkstra(&adj, &mut dx, x-1);
    let mut dr = vec![INF; n];
    dijkstra(&rev_adj, &mut dr, x-1);

    let mut ans = 0;
    (0..n).for_each(|i| {
        ans = ans.max(dx[i] + dr[i]);
    });
    writeln!(so, "{}", ans).ok();
}
