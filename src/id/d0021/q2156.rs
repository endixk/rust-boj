// BOJ 2156 [Wine Tasting]
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
    let a = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let mut dp = vec![0; n];

    if n == 1 { writeln!(so, "{}", a[0]).ok(); return; }
    else { dp[0] = a[0]; }
    if n == 2 { writeln!(so, "{}", a[0] + a[1]).ok(); return; }
    else { dp[1] = a[0] + a[1]; }
    if n == 3 { writeln!(so, "{}", (a[0] + a[1]).max(a[0] + a[2]).max(a[1] + a[2])).ok(); return; }
    else { dp[2] = (a[0] + a[1]).max(a[0] + a[2]).max(a[1] + a[2]); }

    for i in 3..n {
        dp[i] = dp[i - 1].max(dp[i - 2] + a[i]);
        dp[i] = dp[i].max(dp[i - 3] + a[i - 1] + a[i]);
    }
    writeln!(so, "{}", dp.iter().max().unwrap()).ok();
}
