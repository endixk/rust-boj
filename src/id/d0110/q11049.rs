// BOJ 11049 [Matrix Multiplication Order]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn solve(dp: &mut [u32], rv: &[u32], cv: &[u32], n: usize, s: usize, e: usize) -> u32 {
    if dp[s*n+e] != 0 { return dp[s*n+e]; }
    if s == e { return 0; }

    let mut ret = u32::MAX;
    for i in s..e {
        let l = solve(dp, rv, cv, n, s, i);
        let r = solve(dp, rv, cv, n, i + 1, e);
        let m = rv[s] * cv[i] * cv[e];
        ret = ret.min(l + r + m);
    }

    dp[s*n+e] = ret;
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);

    let mut rv = [0; 555];
    let mut cv = [0; 555];
    for i in 0..n {
        rv[i] = next::<u32>(&mut it);
        cv[i] = next::<u32>(&mut it);
    }

    let mut dp = [0; 255555];
    writeln!(so, "{}", solve(&mut dp, &rv, &cv, n, 0, n - 1)).unwrap();
}