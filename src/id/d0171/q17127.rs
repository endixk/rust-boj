// BOJ 17127 [Cherry Blossom]
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

#[inline] fn stat(a: &[i32], i: usize, j: usize) -> (i32, i32) {
    let (mut x, mut y) = (0, 1);
    for &k in &a[i..j] { x += k; y *= k; }
    (x, y)
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let s = a.iter().sum::<i32>();
    let mut ans = 0;
    for i in 0..4 {
        let (x, y) = stat(&a, i, i+n-3);
        ans = ans.max(s - x + y);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
