// BOJ 1912 [Consecutive Sum]
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

    let n = next::<usize>(&mut it);
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let (mut dp, mut ans) = (vec![0; n], a[0]);
    dp[0] = a[0];
    for i in 1..n {
        dp[i] = a[i].max(dp[i-1] + a[i]);
        ans = ans.max(dp[i]);
    }
    writeln!(so, "{}", ans).unwrap();
}
