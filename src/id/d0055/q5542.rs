// BOJ 5542 [Festivals in JOI Kingdom]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read<T>(si: &mut io::BufReader<T>) -> String where T: Read {
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

fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, fest: &Vec<usize>) {
    let mut pq = BinaryHeap::new();
    for src in fest {
        pq.push((Reverse(0), *src));
        dist[*src] = 0;
    }

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

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

fn pbs(adj: &Vec<Vec<(usize, usize)>>,
       dv: &Vec<(usize, usize)>,
       queries: &Vec<(usize, usize)>, n: usize, q: usize) -> Vec<usize> {
    let mut uniq = dv.iter().map(|(_, d)| *d).collect::<Vec<usize>>();
    uniq.dedup();

    let mut lo = vec![0 as usize; q];
    let mut hi = vec![uniq.len(); q];
    let mut rt = vec![0 as usize; q];

    loop {
        let mut idx = vec![vec![]; uniq.len()];
        let mut cnt = 0;
        lo.iter().zip(hi.iter()).enumerate().for_each(|(i, (l, h))| {
            if *l+1 < *h {
                idx[(*l+*h)>>1].push(i);
                cnt += 1;
            }
        });
        if cnt == 0 { break }

        let mut root = (0..n+1).collect::<Vec<usize>>();
        let mut rank = vec![0; n+1];
        let mut open = vec![false; n+1];
        let mut x = 0;
        for (v, d) in dv {
            if *d < uniq[x] {
                for i in idx[x].iter() {
                    let (u, v) = queries[*i];
                    if find(&mut root, u) == find(&mut root, v) {
                        rt[*i] = uniq[x];
                        hi[*i] = x;
                    } else {
                        lo[*i] = x;
                    }
                }
                x += 1;
            }

            open[*v] = true;
            for (u, _) in &adj[*v] {
                if open[*u] {
                    union(&mut root, &mut rank, *u, *v);
                }
            }
        }
        for i in idx[x].iter() {
            let (u, v) = queries[*i];
            if find(&mut root, u) == find(&mut root, v) {
                rt[*i] = uniq[x];
                hi[*i] = x;
            } else {
                lo[*i] = x;
            }
        }
    }

    rt
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m, k, q) = (
        next::<usize>(&mut it), next::<usize>(&mut it),
        next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let fest = (0..k).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let queries = (0..q).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();

    // Dijkstra
    let mut dist = vec![INF; n+1];
    dijkstra(&adj, &mut dist, &fest);
    let mut dv = vec![];
    for (i, d) in dist.iter().skip(1).enumerate().filter(|(_, d)| **d > 0) {
        dv.push((i+1, *d));
    }
    dv.sort_by_key(|(_, d)| Reverse(*d));

    pbs(&adj, &dv, &queries, n, q).iter().for_each(|&d| writeln!(so, "{}", d).unwrap());
}
