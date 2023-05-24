// BOJ 17070 [Moving a Pipe 1]
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

fn solve(dp: &mut Vec<Vec<Vec<i32>>>, a: &Vec<Vec<bool>>, i: usize, j: usize, ori: usize) -> i32 {
    if i == 0 || j == 0 { return 0; }
    if i == 1 && j == 1 && ori == 0 { return 1; }
    if dp[i][j][ori] >= 0 { return dp[i][j][ori]; }

    dp[i][j][ori] = 0;
    if !a[i][j] { return 0; }
    if ori == 0 { // horizontal
        if !a[i][j+1] { return 0; }
        dp[i][j][ori] += solve(dp, a, i, j-1, 0);
        dp[i][j][ori] += solve(dp, a, i-1, j-1, 2);
    } else if ori == 1 { // vertical
        if !a[i+1][j] { return 0; }
        dp[i][j][ori] += solve(dp, a, i-1, j, 1);
        dp[i][j][ori] += solve(dp, a, i-1, j-1, 2);
    } else { // diagonal
        if !a[i][j+1] || !a[i+1][j] || !a[i+1][j+1] { return 0; }
        dp[i][j][ori] += solve(dp, a, i, j-1, 0);
        dp[i][j][ori] += solve(dp, a, i-1, j, 1);
        dp[i][j][ori] += solve(dp, a, i-1, j-1, 2);
    }

    dp[i][j][ori]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    let mut a = vec![vec![false; n+1]; n+1];
    for i in 1..=n { for j in 1..=n { a[i][j] = next::<i8>(&mut it) == 0; } }

    let mut dp = vec![vec![vec![-1; 3]; n+1]; n+1];
    writeln!(so, "{}",
             solve(&mut dp, &a, n, n-1, 0) +
             solve(&mut dp, &a, n-1, n, 1) +
             solve(&mut dp, &a, n-1, n-1, 2)).ok();
}
