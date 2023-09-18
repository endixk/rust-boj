// BOJ 25200 [Vending Machine]
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
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let e = (0..m).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();

    let mut a = (0..=n).collect::<Vec<_>>();
    for &(u, v) in e.iter().rev() { a[u] = a[v]; }
    a.iter().skip(1).for_each(|&x| write!(so, "{} ", x).unwrap());
}
