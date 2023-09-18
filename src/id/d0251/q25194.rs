// BOJ 25194 [Friday]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut dp = vec![false; 7];
    dp[0] = true;
    for _ in 0..next(&mut it) {
        let a = next::<usize>(&mut it);
        let mut tp = dp.clone();
        for i in 0..7 {
            tp[(i+a)%7] |= dp[i];
        }
        dp = tp;
    }
    writeln!(so, "{}", if dp[4] { "YES" } else { "NO" }).unwrap();
}
