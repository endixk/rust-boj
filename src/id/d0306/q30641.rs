// BOJ 30641 [Palindrome Word Chain]
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

    const MOD: usize = 1_000_000_007;
    let (l, r) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut ans, mut p) = (0, 1);
    for i in 1..=r {
        if i >= l { ans = (ans + p) % MOD; }
        if i & 1 == 0 { p = p * 26 % MOD; }
    }
    writeln!(so, "{} {}", if l == 2 || r == 1 { "H" } else { "A" }, ans)?;

    Ok(())
}
