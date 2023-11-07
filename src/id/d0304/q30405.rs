// BOJ 30405 [Museum Tour]
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

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = vec![];
    for _ in 0..n {
        let k = next::<usize>(&mut it);
        for i in 0..k {
            let x = next::<usize>(&mut it);
            if i == 0 || i == k-1 { v.push(x); }
        }
    }
    v.sort_unstable();
    writeln!(so, "{}", v[n-1])?;

    Ok(())
}
