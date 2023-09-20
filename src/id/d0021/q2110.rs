// BOJ 2110 [Aggressive Cows]
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

fn go(x: &[usize], c: usize, d: usize) -> bool {
    let (mut n, mut p) = (1, x[0]);
    for i in 1..x.len() {
        if x[i] - p >= d { n += 1; p = x[i]; }
        if n >= c { return true; }
    }
    false
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut x = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    x.sort_unstable();

    let (mut l, mut r) = (1, x[n - 1] - x[0]);
    while l <= r {
        let m = (l + r) / 2;
        if go(&x, c, m) { l = m + 1; }
        else { r = m - 1; }
    }
    writeln!(so, "{}", r)?;

    Ok(())
}
