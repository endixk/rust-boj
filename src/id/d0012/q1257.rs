// BOJ 1257 [Super Rich]
// Supported by GitHub Copilot

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io;
use std::io::prelude::*;

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

fn dijkstra(v: &Vec<usize>, n: usize, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for &x in v[..n-1].iter() {
            if u + x >= v[n-1] {
                if dist[u + x - v[n-1]] > dist[u] {
                    dist[u + x - v[n-1]] = dist[u];
                    pq.push((Reverse(dist[u + x - v[n-1]]), u + x - v[n-1]));
                }
            } else if dist[u + x] > dist[u] + 1 {
                dist[u + x] = dist[u] + 1;
                pq.push((Reverse(dist[u + x]), u + x));
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (m, n) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    v.sort_unstable();

    const INF: usize = 0x3f3f3f3f;
    let mut dist = vec![INF; v[n-1]];
    dijkstra(&v, n, &mut dist, 0);

    writeln!(so, "{}", dist[m % v[n-1]] + m / v[n-1]).ok();
}
