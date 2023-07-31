// BOJ 11728 [Merge Arrays]
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
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let b = (0..m).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let (mut i, mut j) = (0, 0);
    loop {
        if i == n || j == m { break; }
        if a[i] < b[j] { write!(so, "{} ", a[i]).unwrap(); i += 1; }
        else { write!(so, "{} ", b[j]).unwrap(); j += 1; }
    }
    for i in i..n { write!(so, "{} ", a[i]).unwrap(); }
    for j in j..m { write!(so, "{} ", b[j]).unwrap(); }
}
