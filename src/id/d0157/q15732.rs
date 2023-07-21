// BOJ 15732 [Hide the Acorns]
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

fn go(rules: &[(usize, usize, usize)], d: usize, x: usize) -> bool {
    let mut cnt = 0;
    for &(s, e, i) in rules {
        if x < s { continue; }
        if x > e { cnt += (e - s) / i + 1; }
        else { cnt += (x - s) / i + 1; }
    }
    cnt >= d
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k, d) = (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it));
    let rules = (0..k).map(|_| (
        next::<usize>(&mut it),
        next::<usize>(&mut it),
        next::<usize>(&mut it))).collect::<Vec<_>>();
    let (mut l, mut r) = (1, n);
    while l < r {
        let m = (l + r) / 2;
        if go(&rules, d, m) { r = m; }
        else { l = m + 1; }
    }
    println!("{}", l);
}
