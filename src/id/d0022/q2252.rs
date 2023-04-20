// BOJ 2252 [Lining Up]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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
    let mut adj = vec![vec![]; n];
    let mut ind = vec![0; n];
    for _ in 0..m {
        let (a, b) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
        adj[a].push(b);
        ind[b] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if ind[i] == 0 {
            q.push_back(i);
        }
    }

    let mut vis = vec![false; n];
    while let Some(i) = q.pop_front() {
        vis[i] = true;
        write!(so, "{} ", i + 1).unwrap();
        for &j in &adj[i] {
            ind[j] -= 1;
            if ind[j] == 0 {
                q.push_back(j);
            }
        }
    }
}
