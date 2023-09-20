// BOJ 2629 [Balance]
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
    let a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut dp = vec![false; 30003];
    dp[15000] = true;
    for w in a {
        let mut tp = dp.clone();
        for i in 0..dp.len() {
            if i >= w { tp[i-w] |= dp[i]; }
            if i + w < dp.len() { tp[i+w] |= dp[i]; }
        }
        dp = tp;
    }

    for _ in 0..next(&mut it) {
        let w = next::<usize>(&mut it);
        if w > 15000 { write!(so, "N ")?; }
        else { write!(so, "{}", if dp[15000-w] | dp[15000+w] { "Y " } else { "N " })?; }
    }
    writeln!(so)?;

    Ok(())
}
