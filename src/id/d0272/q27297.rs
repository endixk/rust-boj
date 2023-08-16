// BOJ 27297 [Party at Manhattan]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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
    let mut so = io::BufWriter::with_capacity(1<<20, io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = vec![vec![]; n];
    for _ in 0..m {
        for i in 0..n {
            v[i].push(next::<i64>(&mut it));
        }
    }

    let mut ans = 0;
    for i in 0..n {
        v[i].sort_unstable();
        let med = v[i][m / 2];
        for &x in &v[i] {
            ans += (x - med).abs();
        }
        write!(so, "{} ", med).ok();
    }
    println!("{}", ans);
}
