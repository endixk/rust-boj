// BOJ 1647 [City Division Plan]
// Supported by GitHub Copilot

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

fn kruskal(edges: Vec<(usize, usize, usize)>, n: usize) -> usize {
    let mut root = vec![0; n + 1];
    let mut rank = vec![0; n + 1];
    for i in 1..=n {
        root[i] = i;
    }
    let mut mst = 0;
    let mut maxw = 0;
    for (w, u, v) in edges {
        if find(&mut root, u) != find(&mut root, v) {
            union(&mut root, &mut rank, u, v);
            mst += w;
            maxw = maxw.max(w);
        }
    }
    mst - maxw
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut edges = (0..m).map(|_| {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        (w, u, v)
    }).collect::<Vec<_>>();
    edges.sort_unstable();
    writeln!(so, "{}", kruskal(edges, n)).unwrap();
}
