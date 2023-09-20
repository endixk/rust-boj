// BOJ 1300 [Kth Number]
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

fn get(x: usize, n: usize) -> usize {
    let mut ret = 0;
    for i in 0..n { ret += n.min(x / (i+1)); }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut l, mut r) = (1, n*n);
    while l < r {
        let m = (l+r)/2;
        if get(m, n) < k { l = m+1; }
        else { r = m; }
    }
    writeln!(so, "{}", l)?;

    Ok(())
}
