// BOJ 11001 [Kimchi]
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

fn dnco(dp: &mut [usize], t: &[usize], v: &[usize],
        s: usize, e: usize, l: usize, r: usize, d: usize) {
    if s > e { return; }
    let m = (s + e) >> 1;

    let (mut x, mut xi) = (0, 0);
    for i in l.max(if m>d {m-d} else {1})..=r.min(m) {
        let k = (m - i) * t[m-1] + v[i-1];
        if x < k { x = k; xi = i; }
    }

    dp[m-1] = x;
    dnco(dp, t, v, s, m-1, l, xi, d);
    dnco(dp, t, v, m+1, e, xi, r, d);
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, d) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let t = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut dp = vec![0; n];
    dnco(&mut dp, &t, &v, 1, n, 1, n, d);
    println!("{}", dp.iter().max().unwrap());
}
