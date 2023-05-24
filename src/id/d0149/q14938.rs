// BOJ 14938 [SogangGrounds]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, r) = (next::<usize>(&mut it), next::<u16>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<u16>(&mut it)).collect::<Vec<_>>();

    let mut dist = vec![vec![u16::MAX; n]; n];
    for i in 0..n { dist[i][i] = 0; }
    (0..r).for_each(|_| {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<u16>(&mut it));
        dist[u-1][v-1] = w;
        dist[v-1][u-1] = w;
    });

    // Floyd-Warshall
    for k in 0..n { for i in 0..n { for j in 0..n {
        if dist[i][j] > dist[i][k].saturating_add(dist[k][j]) {
            dist[i][j] = dist[i][k].saturating_add(dist[k][j]);
        }
    }}}

    let mut ans = 0;
    (0..n).for_each(|i| {
        let sum = (0..n).filter(|&j| dist[i][j] <= m).map(|j| a[j]).sum::<u16>();
        ans = ans.max(sum);
    });
    writeln!(so, "{}", ans).ok();
}
