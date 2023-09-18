// BOJ 25198 [Errand]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

fn dfs(adj: &Vec<Vec<usize>>, par: &mut [usize], u: usize) {
    for &v in &adj[u] {
        if v != par[u] {
            par[v] = u;
            dfs(adj, par, v);
        }
    }
}
fn lca(par: &[usize], u: usize, v: usize) -> usize {
    let mut up = Vec::new();
    let mut u = u;
    while u > 0 {
        up.push(u);
        u = par[u];
    }

    let mut vp = Vec::new();
    let mut v = v;
    while v > 0 {
        vp.push(v);
        v = par[v];
    }

    let mut l = 0;
    while !up.is_empty() && !vp.is_empty() && up.last().unwrap() == vp.last().unwrap() {
        l = up.pop().unwrap();
        vp.pop();
    }
    l
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (s, c, h) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; n + 1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut par = vec![0; n + 1];
    dfs(&adj, &mut par, c);
    let l = lca(&par, s, h);

    let (mut x, mut y, mut z) = (0i64, 0i64, 0i64);
    let mut u = s;
    while u != l { x += 1; u = par[u]; }
    u = l;
    while u != c { y += 1; u = par[u]; }
    u = h;
    while u != l { z += 1; u = par[u]; }

    writeln!(so, "{}", x*(x-1)/2 + y*(y-1) + z*(z-1)/2 + x + 2*y + z + x*y + y*z + z*x).unwrap();
}
