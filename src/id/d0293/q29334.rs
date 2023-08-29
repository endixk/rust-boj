// BOJ 29334 [Pension]
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

    let n = next::<usize>(&mut it);
    let q = (0..n).map(|_| next::<f64>(&mut it)).collect::<Vec<_>>();
    let r = (0..n).map(|_| next::<f64>(&mut it)).collect::<Vec<_>>();

    let mut t = vec![0.0; n];
    t[n-1] = r[n-1];
    for i in (0..n-1).rev() {
        t[i] = t[i+1] + r[i];
    }

    let (mut p, mut c) = (vec![0.0; n], 0.0);
    for i in 0..n {
        p[i] = q[i] / t[i];
        c += p[i];
    }

    p.iter().for_each(|x| write!(so, "{:.9} ", x / c).unwrap());
}
