// BOJ 30015 [Student Council]
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

fn get(a: &[i32], b: u8) -> Vec<i32> {
    let mut ret = Vec::new();
    for &x in a {
        if x & (1 << b) != 0 { ret.push(x) }
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut ans = 0;
    for b in (0..20).rev() {
        let v = get(&a, b);
        if v.len() >= k {
            ans += 1 << b;
            a = v;
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
