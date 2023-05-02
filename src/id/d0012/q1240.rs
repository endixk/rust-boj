// BOJ 1240 [Distance Between the Nodes]
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

fn generate(adj: &[Vec<(usize, i32)>], dep: &mut [i32], dis: &mut [i32], par: &mut [Vec<i32>],
            c: usize, p: usize, d: i32, s: i32) {
    dep[c] = d;
    dis[c] = s;
    par[c][0] = p as i32;
    for i in 1..21 {
        par[c][i] = par[par[c][i - 1] as usize][i - 1];
    }

    for (n, w) in &adj[c] {
        if *n == p { continue; }
        generate(adj, dep, dis, par, *n, c, d + 1, s + w);
    }
}

fn lca(dep: &[i32], par: &[Vec<i32>], mut a: usize, mut b: usize) -> usize {
    if dep[a] > dep[b] {
        let t = a; a = b; b = t;
    }
    for i in (0..21).rev() {
        if dep[b] - dep[a] >= (1 << i) {
            b = par[b][i] as usize;
        }
    }
    if a == b { return a; }
    for i in (0..21).rev() {
        if par[a][i] != par[b][i] {
            a = par[a][i] as usize;
            b = par[b][i] as usize;
        }
    }
    par[a][0] as usize
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut adj = vec![vec![]; n + 1];
    (1..n).for_each(|_| {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    });

    let mut dep = vec![0; n + 1]; // depth from root
    let mut dis = vec![0; n + 1]; // distance from root
    let mut par = vec![vec![0; 21]; n + 1]; // parent table
    generate(&adj, &mut dep, &mut dis, &mut par, 1, 0, 0, 0);

    (0..m).for_each(|_| {
        let (a, b) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let p = lca(&dep, &par, a, b);
        writeln!(so, "{}", dis[a] + dis[b] - 2 * dis[p]).unwrap();
    });
}
