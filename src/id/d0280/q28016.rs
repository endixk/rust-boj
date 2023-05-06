// BOJ 28016 [Prize Drawing]
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; m]; n];
    let mut s = (0, 0);
    for i in 0..n {
        for j in 0..m {
            a[i][j] = next::<u8>(&mut it);
            if a[i][j] == 2 {
                s = (i, j);
                a[i][j] = 0;
            }
        }
    }

    let mut dp = vec![vec![0u128; m]; n];
    dp[s.0][s.1] = 1<<n;
    for i in 0..n-1 {
        for j in 0..m {
            if dp[i][j] == 0 { continue; }
            if a[i+1][j] == 0 { dp[i+1][j] += dp[i][j]; }
            else {
                if a[i][j-1] == 0 && a[i+1][j-1] == 0 {
                    dp[i+1][j-1] += dp[i][j] >> 1;
                }
                if a[i][j+1] == 0 && a[i+1][j+1] == 0 {
                    dp[i+1][j+1] += dp[i][j] >> 1;
                }
            }
        }
    }

    let (mut a, mut i) = (0u128, 0);
    for j in 0..m {
        if dp[n-1][j] > a {
            a = dp[n-1][j];
            i = j;
        }
    }
    println!("{}", if a == 0 { -1 } else { i as i32 });
}
