// BOJ 1766 [Workbook]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; n + 1];
    let mut ind = vec![0; n + 1];
    for _ in 0..m {
        let (a, b) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[a].push(b);
        ind[b] += 1;
    }

    let mut pq = BinaryHeap::new();
    ind.iter().enumerate().skip(1).filter(|(_, &x)| x == 0).for_each(|(i, _)| pq.push(Reverse(i)));
    while let Some(Reverse(x)) = pq.pop() {
        write!(so, "{} ", x).unwrap();
        for &y in &adj[x] {
            ind[y] -= 1;
            if ind[y] == 0 {
                pq.push(Reverse(y));
            }
        }
    }
}
