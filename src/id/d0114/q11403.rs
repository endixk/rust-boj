// BOJ 11403 [Finding Paths]
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

fn dfs(adj: &Vec<Vec<usize>>, reach: &mut Vec<Vec<bool>>, p: usize, v: usize) {
    for &u in &adj[v] {
        if !reach[p][u] {
            reach[p][u] = true;
            dfs(adj, reach, p, u);
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n];
    for i in 0..n {
        (0..n).map(|_| next::<u8>(&mut it)).enumerate()
            .filter(|&(_, b)| b > 0)
            .for_each(|(j, _)| adj[i].push(j));
    }

    let mut reach = vec![vec![false; n]; n];
    (0..n).for_each(|i| dfs(&adj, &mut reach, i, i));
    (0..n).for_each(|i| {
        (0..n).for_each(|j| {
            write!(so, "{} ", if reach[i][j] { 1 } else { 0 }).unwrap();
        });
        writeln!(so).unwrap();
    });
}
