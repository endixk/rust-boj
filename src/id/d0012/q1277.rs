// BOJ 1277 [Power Failure]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
struct sf64(f64);
impl Eq for sf64 {}
impl PartialEq for sf64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Ord for sf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 { Ordering::Less }
        else if self.0 > other.0 { Ordering::Greater }
        else { Ordering::Equal }
    }
}
impl PartialOrd for sf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

fn dijkstra(adj: &Vec<Vec<sf64>>, dist: &mut Vec<sf64>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(sf64(0.0)), src));
    dist[src] = sf64(0.0);
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for (v, &w) in adj[u].iter().enumerate() {
            if w.0 < 0.0 { continue; }
            let next = sf64(dist[u].0 + w.0);
            if dist[v] > next {
                dist[v] = next;
                pq.push((Reverse(dist[v]), v));
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, w, m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<f64>(&mut it));
    let mut adj = vec![vec![sf64(-1.0); n]; n];

    let co = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    for i in 0..n { for j in 0..n {
        if i == j { continue; }
        let d = (((co[i].0 - co[j].0).pow(2) + (co[i].1 - co[j].1).pow(2)) as f64).sqrt();
        if d <= m { adj[i][j] = sf64(d); }
    }}

    (0..w).for_each(|_| {
        let (u, v) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
        adj[u][v] = sf64(0.0); adj[v][u] = sf64(0.0);
    });

    let mut dist = vec![sf64(1e100); n];
    dijkstra(&adj, &mut dist, 0);
    let ans = (dist[n-1].0 * 1000.0).floor();
    writeln!(so, "{}", if ans > 1e100 { -1.0 } else { ans }).unwrap();
}
