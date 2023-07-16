// BOJ 2546 [Economics]
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

    for _ in 0..next(&mut it) {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        let w = (0..m).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        let (vs, ws) = (v.iter().sum::<usize>(), w.iter().sum::<usize>());
        writeln!(so, "{}", v.iter().filter(|&&x| x * n < vs && x * m > ws).count()).unwrap();
    }
}
