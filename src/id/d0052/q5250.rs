// BOJ 5250 [Shortest Paths]
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

fn span(adj: &Vec<Vec<(usize, usize)>>,
        dist: &Vec<usize>,
        inv_lucky: &Vec<usize>,
        par: &mut Vec<usize>, u: usize, p: usize) {
    if par[u] > 0 { return; }
    let lucky = inv_lucky[u] > 0;
    let tp = if lucky { u } else { p };
    par[u] = tp;

    for (v, w) in &adj[u] {
        if !lucky && inv_lucky[*v] > 0 { continue; } // force the lucky path to be included
        if par[*v] > 0 { continue; } // already visited
        if dist[*v] == dist[u] + *w {
            span(adj, dist, inv_lucky, par, *v, tp);
        }
    }
}

// Minimum Segment Tree
struct SegTree {
    n: usize,
    v: Vec<usize>,
    lazy: Vec<usize>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![INF; m<<1], lazy: vec![INF; m<<1] }
    }

    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == INF { return; }
        self.v[x] = self.v[x].min(self.lazy[x]);
        if s < e {
            self.lazy[x<<1] = self.lazy[x].min(self.lazy[x<<1]);
            self.lazy[x<<1|1] = self.lazy[x].min(self.lazy[x<<1|1]);
        }
        self.lazy[x] = INF;
    }

    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] = v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            self.v[x] = self.v[x<<1].min(self.v[x<<1|1]);
        }
    }

    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> usize {
        self.propagate(x, s, e);
        if r < s || e < l { return INF; }
        if l <= s && e <= r {
            self.v[x]
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).min(self.query(x<<1|1, m+1, e, l, r))
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (src, dst) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![Vec::<(usize, usize)>::with_capacity(n+1); n+1];
    (0..m).for_each(|_| {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    });

    let mut lucky = vec![0];
    let mut inv_lucky = vec![0; n+1];
    for i in 0..next::<usize>(&mut it) {
        let v = next::<usize>(&mut it);
        lucky.push(v);
        inv_lucky[v] = i+1;
    }
    lucky.push(0);


    let mut dist_src = vec![INF; n+1];
    dijkstra(&adj, &mut dist_src, src);
    let mut par_src = vec![0; n+1];
    span(&adj, &dist_src, &inv_lucky, &mut par_src, src, src);

    let mut dist_dst = vec![INF; n+1];
    dijkstra(&adj, &mut dist_dst, dst);
    let mut par_dst = vec![0; n+1];
    span(&adj, &dist_dst, &inv_lucky, &mut par_dst, dst, dst);

    let mut seg = SegTree::new(n+1);
    for u in 1..=n {
        for (v, w) in &adj[u] {
            // continue if (u, v) is a lucky edge
            if inv_lucky[u] > 0 && inv_lucky[*v] > 0 {
                if lucky[inv_lucky[u]+1] == *v || lucky[inv_lucky[*v]+1] == u { continue; }
            }
            let d = dist_src[u] + *w + dist_dst[*v];
            seg.update(1, 1, n, inv_lucky[par_src[u]], inv_lucky[par_dst[*v]]-1, d);
        }
    }

    for i in 1..lucky.len()-2 {
        writeln!(so, "{}", match seg.query(1, 1, n, i, i) {
            INF => -1,
            x => x as i32,
        }).ok();
    }
}
