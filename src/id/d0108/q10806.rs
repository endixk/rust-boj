// BOJ 10806 [Floating Cities]
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

fn dfs(u: usize, p: usize, n: usize, adj: &Vec<Vec<usize>>,
       root: &mut Vec<usize>, rank: &mut Vec<usize>,
       dfsn: &mut Vec<usize>, cnt: &mut usize, arts: &mut Vec<(usize, usize)>) -> usize {
    *cnt += 1;
    dfsn[u] = *cnt;
    let mut ret = dfsn[u];
    let mut vp = true;
    for &v in adj[u].iter() {
        if v == p {
            if vp { vp = false; continue; }
        }
        if dfsn[v] == 0 {
            let prev = dfs(v, u, n, adj, root, rank, dfsn, cnt, arts);
            if prev > dfsn[u] { arts.push((u, v)); }
            else { union(root, rank, u, v); }
            ret = ret.min(prev);
        } else {
            ret = ret.min(dfsn[v]);
        }
    }
    ret
}

fn dfs_leaf(u: usize, p: usize, adj: &Vec<Vec<usize>>, sgl: &Vec<bool>, leaves: &mut Vec<usize>) {
    if sgl[u] { leaves.push(u); }
    for &v in adj[u].iter() {
        if v == p { continue; }
        dfs_leaf(v, u, adj, sgl, leaves);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; n + 1];
    (0..m).for_each(|_| {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    });

    let mut arts = Vec::new();
    let mut dfsn = vec![0; n + 1];
    let mut root = (0..=n).collect::<Vec<usize>>();
    let mut rank = vec![0; n + 1];
    let mut cnt = 0;
    for u in 1..=n {
        if dfsn[u] == 0 {
            dfs(u, 0, n, &adj, &mut root, &mut rank, &mut dfsn, &mut cnt, &mut arts);
        }
    }

    adj = vec![vec![]; n + 1];
    for &(u, v) in arts.iter() {
        if find(&mut root, u) != find(&mut root, v) {
            adj[find(&mut root, u)].push(find(&mut root, v));
            adj[find(&mut root, v)].push(find(&mut root, u));
        }
    }

    let mut sgl = vec![false; n + 1];
    let mut cnt = 0;
    for u in 1..=n {
        if adj[u].len() == 1 {
            sgl[u] = true;
            cnt += 1;
        }
    }
    writeln!(so, "{}", (cnt + 1) / 2).ok();
    if cnt == 0 { return; }

    let mut leaves = Vec::new();
    dfs_leaf(find(&mut root, 1), 0, &adj, &sgl, &mut leaves);
    for i in 0..cnt/2 {
        writeln!(so, "{} {}", leaves[i], leaves[i + (cnt+1)/2]).ok();
    }
    if cnt % 2 == 1 {
        writeln!(so, "{} {}", leaves[0], leaves[cnt/2]).ok();
    }
}
