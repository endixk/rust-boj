// BOJ 30012 [Matching Frogs]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (p, n) = (next::<i32>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let (k, l) = (next::<i32>(&mut it), next::<i32>(&mut it));

    let (mut ans, mut ai) = (0x3f3f3f3f, 0);
    for i in 0..n {
        let d = (p - a[i]).abs();
        let c = if d > 2*k { d*l - 2*k*l } else { 2*k - d };
        if c < ans { ans = c; ai = i+1; }
    }
    writeln!(so, "{} {}", ans, ai)?;

    Ok(())
}
