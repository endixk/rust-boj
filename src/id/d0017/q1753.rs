// BOJ 1753 [Shortest Paths]
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

const INF: u32 = 0x3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(usize, u8)>>, dist: &mut Vec<u32>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;

    while let Some((Reverse(d), u)) = pq.pop() {
        if d > dist[u] { continue; }
        for (v, w) in &adj[u] {
            if dist[u] + (*w as u32) < dist[*v] {
                dist[*v] = dist[u] + *w as u32;
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
    let v: Vec<usize> = it.by_ref().take(3).map(|x| x.parse().unwrap()).collect();

    let mut adj = vec![vec![]; v[0]+1];
    let mut dist = vec![INF; v[0]+1];
    for _ in 0..v[1] {
        let p: usize = it.by_ref().next().unwrap().parse().unwrap();
        let q: usize = it.by_ref().next().unwrap().parse().unwrap();
        let w: u8 = it.by_ref().next().unwrap().parse().unwrap();
        adj[p].push((q, w));
    }

    dijkstra(&adj, &mut dist, v[2]);
    dist.iter().skip(1).map(|x|
        if *x == INF { String::from("INF") }
        else { x.to_string() }
    ).for_each(|x| writeln!(so, "{}", x).unwrap());
}
