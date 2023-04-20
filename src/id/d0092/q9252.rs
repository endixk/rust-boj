// BOJ 9252 [LCS 2]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let sx = next::<String>(&mut it).chars().collect::<Vec<char>>();
    let sy = next::<String>(&mut it).chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0; sy.len() + 1]; sx.len() + 1];
    let mut tr = vec![vec![0; sy.len() + 1]; sx.len() + 1]; // 0: left, 1: up, 2: diag
    for i in 1..=sx.len() {
        for j in 1..=sy.len() {
            if sx[i - 1] == sy[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                tr[i][j] = 2;
            } else {
                if dp[i - 1][j] > dp[i][j - 1] {
                    dp[i][j] = dp[i - 1][j];
                    tr[i][j] = 1;
                } else {
                    dp[i][j] = dp[i][j - 1];
                    tr[i][j] = 0;
                }
            }
        }
    }

    let (mut i, mut j) = (sx.len(), sy.len());
    let mut ans = String::new();
    while i > 0 && j > 0 {
        if tr[i][j] == 2 {
            ans.push(sx[i - 1]);
            i -= 1;
            j -= 1;
        } else if tr[i][j] == 1 {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    writeln!(so, "{}", dp[sx.len()][sy.len()]).unwrap();
    writeln!(so, "{}", ans.chars().rev().collect::<String>()).unwrap();
}
