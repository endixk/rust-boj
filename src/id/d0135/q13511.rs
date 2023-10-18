// BOJ 13511 [Tree and Queries 2]
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

fn dfs(dep: &mut Vec<usize>, par: &mut Vec<Vec<usize>>, dst: &mut Vec<usize>,
       adj: &Vec<Vec<(usize, usize)>>, u: usize, p: usize, d: usize) {
    dep[u] = d;
    par[u][0] = p;
    for i in 1..17 {
        par[u][i] = par[par[u][i-1]][i-1];
    }
    for &v in &adj[u] {
        if v.0 != p {
            dst[v.0] = dst[u] + v.1;
            dfs(dep, par, dst, adj, v.0, u, d+1);
        }
    }
}

fn lca(dep: &Vec<usize>, par: &Vec<Vec<usize>>, u: usize, v: usize) -> usize {
    let (mut u, mut v) = (u, v);
    if dep[u] < dep[v] {
        std::mem::swap(&mut u, &mut v);
    }
    let diff = dep[u] - dep[v];
    for i in 0..17 {
        if diff & (1 << i) != 0 {
            u = par[u][i];
        }
    }
    if u != v {
        for i in (0..17).rev() {
            if par[u][i] != par[v][i] {
                u = par[u][i];
                v = par[v][i];
            }
        }
        u = par[u][0];
    }
    u
}

fn dist(dep: &Vec<usize>, par: &Vec<Vec<usize>>, dst: &Vec<usize>, u: usize, v: usize) -> usize {
    dst[u] + dst[v] - 2 * dst[lca(dep, par, u, v)]
}
fn climb(par: &Vec<Vec<usize>>, u: usize, k: usize) -> usize {
    let mut u = u;
    for i in 0..17 {
        if k & (1 << i) != 0 {
            u = par[u][i];
        }
    }
    u
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut dep = vec![0; n+1];
    let mut par = vec![vec![0; 17]; n+1];
    let mut dst = vec![0; n+1];
    for &v in &adj[1] {
        dst[v.0] = v.1;
        dfs(&mut dep, &mut par, &mut dst, &adj, v.0, 1, 1);
    }

    for _ in 0..next(&mut it) {
        let q = next::<u8>(&mut it);
        if q == 1 {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            writeln!(so, "{}", dist(&dep, &par, &dst, u, v))?;
        } else {
            let (u, v, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
            let l = lca(&dep, &par, u, v);
            if k <= dep[u] - dep[l] + 1 {
                writeln!(so, "{}", climb(&par, u, k-1))?;
            } else {
                writeln!(so, "{}", climb(&par, v, dep[u] - dep[l] + dep[v] - dep[l] + 1 - k))?;
            }
        }
    }

    Ok(())
}
