// BOJ 28019 [Travel Plan]
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
fn kruskal(n: usize, mut edges: Vec<(usize, usize, usize)>) -> (Vec<Vec<(usize, usize)>>, usize) {
    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));
    let mut root = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    let mut mst = vec![vec![]; n];
    let mut size = 0;
    while let Some((u, v, w)) = edges.pop() {
        if find(&mut root, u) != find(&mut root, v) {
            union(&mut root, &mut rank, u, v);
            mst[u].push((v, w));
            mst[v].push((u, w));
            size += w;
        }
    }
    (mst, size)
}

fn dfs(adj: &Vec<Vec<(usize, usize)>>, cur: usize, pre: usize) -> usize {
    let mut ret = 0;
    for (nxt, w) in adj[cur].iter() {
        if *nxt == pre { continue; }
        ret = ret.max(dfs(adj, *nxt, cur) + w);
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let edges = (0..m).map(|_| {
        (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, next::<usize>(&mut it))
    });
    let (mst, size) = kruskal(n, edges.collect::<Vec<(usize, usize, usize)>>());

    let src = next::<usize>(&mut it) - 1;
    writeln!(so, "{}", size * 2 - dfs(&mst, src, src)).ok();
}
