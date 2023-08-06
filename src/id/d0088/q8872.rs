// BOJ 8872 [Billabongs]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

fn dfs(dist: &mut [i64], vis: &mut [bool], path: &mut [usize], adj: &Vec<Vec<(usize, i64)>>, x: usize, n: usize) -> i64 {
    vis[x] = true;
    let (mut dx, mut di) = (0, n);
    for &(y, d) in adj[x].iter() {
        if !vis[y] {
            let dy = dfs(dist, vis, path, adj, y, n) + d;
            if dy > dx {
                dx = dy;
                di = y;
            }
        }
    }
    dist[x] = dx;
    path[x] = di;
    dx
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, l) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (a, b, t) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
        adj[a].push((b, t));
        adj[b].push((a, t));
    }

    let mut dist = vec![0; n];
    let mut vis = vec![false; n];
    let mut path = vec![n; n];
    let mut roots = Vec::new();
    for i in 0..n {
        if !vis[i] {
            dfs(&mut dist, &mut vis, &mut path, &adj, i, n);
            let mut j = i;
            while path[j] != n {
                j = path[j];
            }
            roots.push(j);
        }
    }

    dist = vec![0; n];
    vis = vec![false; n];
    path = vec![n; n];
    let mut rads = Vec::new();
    let mut ans = 0;
    for &r in &roots {
        let x = dfs(&mut dist, &mut vis, &mut path, &adj, r, n);
        ans = ans.max(x);
        let (mut j, mut rad) = (r, x);
        while path[j] != n {
            rad = rad.min(dist[j].max(x - dist[j]));
            j = path[j];
        }
        rads.push(rad);
    }

    rads.sort_unstable_by(|a, b| b.cmp(a));
    if rads.len() > 1 {
        ans = ans.max(rads[0] + rads[1] + l);
    }
    if rads.len() > 2 {
        ans = ans.max(rads[1] + rads[2] + 2 * l);
    }
    println!("{}", ans);
}
