// BOJ 2579 [Climbing Stairs]
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

fn solve(v: &Vec<u32>, dp: &mut Vec<Vec<u32>>, n: usize, s: usize) -> u32 {
    return if dp[n][s] > 0 { dp[n][s] }
    else {
        if n == 0 { dp[n][s] = v[0]; }
        else if n == 1 {
            dp[n][s] =  if s == 0 { v[0] + v[1] } else { v[1] };
        }
        else if s == 0 {
            dp[n][s] = solve(v, dp, n - 1, 1) + v[n];
        } else {
            dp[n][s] = solve(v, dp, n - 2, 0).max(solve(v, dp, n - 2, 1)) + v[n];
        }
        dp[n][s]
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);
    let v = (0..n).map(|_| next(&mut it)).collect::<Vec<u32>>();
    let mut dp = vec![vec![0; 2]; n];
    solve(&v, &mut dp, n - 1, 0);
    solve(&v, &mut dp, n - 1, 1);
    writeln!(so, "{}", dp[n-1][0].max(dp[n-1][1])).ok();
}
