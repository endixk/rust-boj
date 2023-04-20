// BOJ 2623 [Music Show]
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
    let (n, m) = (next(&mut it), next(&mut it));

    let mut adj = vec![vec![]; n];
    let mut ind = vec![0; n];
    for _ in 0..m {
        let k = next(&mut it);
        let mut prev = next::<usize>(&mut it) - 1;
        for _ in 1..k {
            let curr = next::<usize>(&mut it) - 1;
            adj[prev].push(curr);
            ind[curr] += 1;
            prev = curr;
        }
    }

    let mut q = VecDeque::new();
    ind.iter().enumerate().for_each(|(i, &x)| if x == 0 { q.push_back(i); });

    let mut ans = vec![];
    while let Some(curr) = q.pop_front() {
        ans.push(curr + 1);
        for &next in &adj[curr] {
            ind[next] -= 1;
            if ind[next] == 0 {
                q.push_back(next);
            }
        }
    }

    if ans.len() < n {
        writeln!(so, "0").unwrap();
    } else {
        ans.iter().for_each(|&x| write!(so, "{} ", x).unwrap());
        writeln!(so).unwrap();
    }
}
