// BOJ 25308 [Radial Graph]
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

fn go(a: &[i32], p: &[usize], n: usize) -> i32 {
    for i in 0..n {
        let (a, b, c) = (a[p[i]], a[p[(i + 1) % n]], a[p[(i + 2) % n]]);
        if (b as f64) < (a * c) as f64 / (a + c) as f64 * 2.0f64.sqrt() { return 0 }
    }
    1
}
fn perm(a: &[i32], p: &mut [usize], v: &mut [bool], c: usize, n: usize) -> i32 {
    if c == n { return go(a, p, n); }
    let mut ret = 0;
    for i in 0..n {
        if !v[i] {
            v[i] = true;
            p[c] = i;
            ret += perm(a, p, v, c + 1, n);
            v[i] = false;
        }
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let a = (0..8).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut p = [0; 8];
    let mut v = [false; 8];
    writeln!(so, "{}", perm(&a, &mut p, &mut v, 0, 8))?;

    Ok(())
}
