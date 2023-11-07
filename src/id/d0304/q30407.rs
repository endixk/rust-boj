// BOJ 30407 [Snack Thief]
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

fn go(n: usize, h: i32, d: i32, k: i32, r: &[i32], i: usize, f: bool, flog: bool) -> i32 {
    if i == n { return if h <= 0 { -1 } else { h } }
    let mut ret = -1;
    let x = if f { 0 } else if d < r[i] { r[i] - d } else { 0 };
    let y = if f { 0 } else if d + k < r[i] { r[i] - d - k } else { 0 };
    if !flog {
        ret = ret.max(go(n, h-x, d, k, r, i+1, true, true));
    }
    ret = ret.max(go(n, h-x/2, d, k, r, i+1, false, flog));
    ret = ret.max(go(n, h-y, d+k, k, r, i+1, false, flog));
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (h, d, k) = (next::<i32>(&mut it), next::<i32>(&mut it), next::<i32>(&mut it));
    let r = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    writeln!(so, "{}", go(n, h, d, k, &r, 0, false, false))?;

    Ok(())
}
