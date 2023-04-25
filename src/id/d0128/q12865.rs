// BOJ 12865 [An Ordinary Knapsack]
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

    let (n, k): (usize, usize) = (next(&mut it), next(&mut it));
    let mut w = vec![0; n];
    let mut v = vec![0; n];
    for i in 0..n {
        (w[i], v[i]) = (next(&mut it), next(&mut it));
    }

    let mut dp = [0; 100001];
    for i in 0..n {
        let mut t = dp.clone();
        for j in 0..k+1 {
            if j >= w[i] {
                t[j] = dp[j].max(dp[j-w[i]] + v[i]);
            }
        }
        dp = t;
    }

    writeln!(so, "{}", dp[k]).unwrap();
}
