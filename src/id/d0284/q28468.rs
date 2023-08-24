// BOJ 28468 [Butterflies 1]
// Supported by GitHub Copilot

use std::io::{self, Read};
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![false; n]; n];
    let mut ind = vec![0; n];
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        adj[u][v] = true; adj[v][u] = true;
        ind[u] += 1; ind[v] += 1;
    }

    let mut ans = 0i64;
    for i in 0..n {
        if ind[i] >= 6 {
            let mut k = 0;
            let a = adj[i].iter().enumerate().filter(|&(_, &x)| x).map(|(j, _)| j).collect::<Vec<_>>();
            let mut sind = vec![0; a.len()];
            let mut cnt = 0;
            for i in 0..a.len()-1 { for j in i+1..a.len() {
                if adj[a[i]][a[j]] {
                    sind[i] += 1; sind[j] += 1;
                    cnt += 1;
                }
            }}
            for i in 0..a.len()-1 { for j in i+1..a.len() {
                if adj[a[i]][a[j]] {
                    k += cnt - sind[i] - sind[j] + 1;
                }
            }}
            ans += k * (ind[i]-4) * (ind[i]-5) / 4;
        }
    }
    println!("{}", ans);
}
