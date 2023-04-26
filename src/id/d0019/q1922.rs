// BOJ 1922 [Network Connections]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

fn kruskal(edges: &Vec<(usize, usize, usize)>, n: usize) -> usize {
    let mut root = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0 as usize; n];

    let mut mst = 0;
    for (w, u, v) in edges {
        if find(&mut root, *u) != find(&mut root, *v) {
            union(&mut root, &mut rank, *u, *v);
            mst += w;
        }
    }
    mst
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = next::<usize>(&mut it);

    let mut edges = Vec::new();
    for _ in 0..next(&mut it) {
        let (u, v, w) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<usize>(&mut it));
        edges.push((w, u, v));
    }

    edges.sort();
    writeln!(so, "{}", kruskal(&edges, n)).unwrap();
}
