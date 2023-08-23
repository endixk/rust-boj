// BOJ 28470 [Attack or Avoid]
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

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let k = (0..n).map(|_| (next::<f64>(&mut it) * 10.0) as i64).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        let (p, q) = (
            a[i] * 10 - b[i] * k[i] + b[i] * k[i] % 10,
            a[i] * k[i] - a[i] * k[i] % 10 - b[i] * 10
        );
        ans += if p > q { p } else { q };
    }
    println!("{}", ans / 10);
}
