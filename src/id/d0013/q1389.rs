// BOJ 1389 [Kevin Bacon's Six Degrees]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    const INF: usize = 0x3f3f3f3f;
    let mut adj = vec![vec![false; n+1]; n+1];
    (0..m).for_each(|_| {
        let (v, w) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[v][w] = true;
        adj[w][v] = true;
    });

    let mut dist = vec![vec![INF; n+1]; n+1];
    (1..=n).for_each(|i| {
        dist[i][i] = 0;
        (1..=n).for_each(|j| {
            if adj[i][j] {
                dist[i][j] = 1;
            }
        });
    });

    // Floyd-Warshall
    (1..=n).for_each(|k| {
        (1..=n).for_each(|i| {
            (1..=n).for_each(|j| {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            });
        });
    });

    let (mut ans, mut min) = (0, INF);
    (1..=n).for_each(|i| {
        let sum = dist[i].iter().skip(1).sum::<usize>();
        if min > sum {
            min = sum;
            ans = i;
        }
    });

    writeln!(so, "{}", ans).unwrap();
}
