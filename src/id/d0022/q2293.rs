// BOJ 2293 [Coin 1]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut dp = vec![0u32; k + 1];
    dp[0] = 1;
    for &c in &v {
        for x in 0..k {
            if x + c <= k {
                dp[x + c] += dp[x];
            }
        }
    }
    writeln!(so, "{}", dp[k]).unwrap();
}
