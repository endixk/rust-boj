// BOJ 2342 [Dance Dance Revolution]
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

fn cost(from: usize, to: usize) -> u32 {
    return if from == to { 1 }
    else if from == 0 { 2 }
    else if from == 1 { if to == 3 { 4 } else { 3 } }
    else if from == 2 { if to == 4 { 4 } else { 3 } }
    else if from == 3 { if to == 1 { 4 } else { 3 } }
    else if from == 4 { if to == 2 { 4 } else { 3 } }
    else { panic!() }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut v = Vec::new();
    loop {
        let n = next::<u8>(&mut it);
        if n == 0 { break } else { v.push(n); }
    }
    let n = v.len();

    const INF: u32 = 0x1f1f1f1f;
    let mut dp = vec![vec![vec![INF; 5]; 5]; n];
    dp[0][0][v[0] as usize] = 2;
    dp[0][v[0] as usize][0] = 2;
    for (i, &x) in v.iter().enumerate().skip(1) {
        let x = x as usize;
        for l in 0..5 { for r in 0..5 {
            if dp[i-1][l][r] == INF { continue }
            if x != r { // move left
                dp[i][x][r] = dp[i][x][r].min(dp[i-1][l][r] + cost(l, x));
            }
            if x != l { // move right
                dp[i][l][x] = dp[i][l][x].min(dp[i-1][l][r] + cost(r, x));
            }
        }}
    }

    let mut ans = INF;
    for l in 0..5 { for r in 0..5 {
        ans = ans.min(dp[n-1][l][r]);
    }}
    writeln!(so, "{}", ans).ok();
}
