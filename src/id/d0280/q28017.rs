// BOJ 28017 [Let's Clear the Game!]
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

const INF: i32 = 0x3f3f3f3f;
fn solve(dp: &mut Vec<Vec<i32>>, a: &Vec<Vec<i32>>, i: usize, j: usize, m: usize) -> i32 {
    if dp[i][j] < INF {
        return dp[i][j];
    }
    if i == 0 { dp[i][j] = a[i][j] }
    else {
        for k in 0..m {
            if j == k { continue; }
            dp[i][j] = dp[i][j].min(solve(dp, a, i - 1, k, m) + a[i][j]);
        }
    }
    dp[i][j]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            a[i][j] = next::<i32>(&mut it);
        }
    }

    let mut dp = vec![vec![INF; m]; n];
    let mut ans = INF;
    for j in 0..m {
        ans = ans.min(solve(&mut dp, &a, n - 1, j, m));
    }
    writeln!(so, "{}", ans).ok();
}

