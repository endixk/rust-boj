// BOJ 1272 [Special Node]
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

fn solve(dp: &mut Vec<Vec<u32>>, w: &Vec<u32>,
         adj: &Vec<Vec<usize>>, v: usize, u: usize, p: usize) -> u32 {
    if dp[v][u] > 0 { return dp[v][u] }

    let mut sp = w[v]; // special
    for &x in adj[v].iter() {
        if x == p { continue; }
        sp += solve(dp, w, adj, x, v, v);
    }

    let mut ns = w[v] - w[u]; // not special
    for &x in adj[v].iter() {
        if x == p { continue; }
        ns += solve(dp, w, adj, x, u, v);
    }

    dp[v][u] = sp.min(ns);
    dp[v][u]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, r) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut w = vec![0; n+1];
    for i in 1..=n { w[i] = next::<u32>(&mut it); }

    let mut adj = vec![vec![]; n+1];
    (1..n).for_each(|_| {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push(v);
        adj[v].push(u);
    });

    let mut dp = vec![vec![0; n+1]; n+1];
    let mut ans = w[r];
    for &x in adj[r].iter() {
        ans += solve(&mut dp, &w, &adj, x, r, r);
    }

    writeln!(so, "{}", ans).ok();
}
