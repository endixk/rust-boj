// BOJ 13305 [Gas Stations]
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

    let n = next::<usize>(&mut it);
    let d = (1..n).map(|_| next::<u64>(&mut it)).collect::<Vec<_>>();
    let p = (0..n).map(|_| next::<u64>(&mut it)).collect::<Vec<_>>();

    let (mut ans, mut mp) = (0, p[0]);
    for i in 1..n {
        ans += mp * d[i-1];
        if mp > p[i] { mp = p[i]; }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
