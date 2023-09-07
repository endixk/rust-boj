// BOJ 27968 [4-Dimensional Candy Bag]
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
    let mut p = vec![0; m+1];
    for i in 1..=m {
        p[i] = p[i-1] + next::<i64>(&mut it);
    }
    for _ in 0..n {
        let x = p.binary_search(&next::<i64>(&mut it)).unwrap_or_else(|x| x);
        if x > m { writeln!(so, "Go away!").ok(); }
        else { writeln!(so, "{}", x).ok(); }
    }
}
