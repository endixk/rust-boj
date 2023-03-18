// BOJ 11404 [Floyd-Warshall]
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

const INF: i32 = 0x3f3f3f3f;
fn floyd(adj: &mut Vec<Vec<i32>>, n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                adj[i][j] = adj[i][j].min(adj[i][k] + adj[k][j]);
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    let mut adj = vec![vec![INF; n]; n];
    for i in 0..n {
        adj[i][i] = 0;
    }
    for _ in 0..next(&mut it) {
        let (a, b, c): (usize, usize, i32) = (next(&mut it), next(&mut it), next(&mut it));
        adj[a - 1][b - 1] = adj[a - 1][b - 1].min(c);
    }

    floyd(&mut adj, n);

    for i in 0..n {
        for j in 0..n {
            if adj[i][j] == INF {
                write!(so, "0 ").unwrap();
            } else {
                write!(so, "{} ", adj[i][j]).unwrap();
            }
        }
        writeln!(so).unwrap();
    }
}
