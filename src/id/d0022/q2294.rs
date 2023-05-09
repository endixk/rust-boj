// BOJ 2294 [Coin 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

    const INF: i32 = 0x3f3f3f3f;
    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut dp = vec![INF; k + 1];

    let mut q = VecDeque::new();
    q.push_back(0usize);
    dp[0] = 0;
    while let Some(x) = q.pop_front() {
        if x == k { break; }
        for &i in &v {
            let nx = x + i;
            if nx <= k && dp[nx] > dp[x] + 1 {
                dp[nx] = dp[x] + 1;
                q.push_back(nx);
            }
        }
    }
    writeln!(so, "{}", if dp[k] == INF { -1 } else { dp[k] }).unwrap();
}
