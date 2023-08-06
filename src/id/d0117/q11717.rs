// BOJ 11717 [Wall Making Game]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

const MAX: usize = 22;
const MEX: u8 = 100;
fn go(dp: &mut [[[[u8; MAX]; MAX]; MAX]; MAX], board: &Vec<Vec<bool>>,
      h1: usize, h2: usize, w1: usize, w2: usize, h: usize, w: usize) -> u8 {
    if h1 < 1 || h2 > h || w1 < 1 || w2 > w { return 0; }
    if h1 > h2 || w1 > w2 { return 0; }
    if h1 == h2 && w1 == w2 {
        return if board[h1][w1] { 1 } else { 0 };
    }
    if dp[h1][h2][w1][w2] < MEX { return dp[h1][h2][w1][w2]; }
    let mut mex = [false; MEX as usize];
    for i in h1..=h2 {
        for j in w1..=w2 {
            if !board[i][j] { continue; }
            mex[(
                go(dp, board, h1, i-1, w1, j-1, h, w) ^
                go(dp, board, h1, i-1, j+1, w2, h, w) ^
                go(dp, board, i+1, h2, w1, j-1, h, w) ^
                go(dp, board, i+1, h2, j+1, w2, h, w)
            ) as usize] = true;
        }
    }
    dp[h1][h2][w1][w2] = mex.iter().position(|&x| !x).unwrap() as u8;
    dp[h1][h2][w1][w2]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (h, w) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![false; w+2]; h+2];
    for i in 0..h {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            b[i+1][j+1] = c == '.';
        }
    }

    let mut dp = [[[[MEX; MAX]; MAX]; MAX]; MAX];
    println!("{}", if go(&mut dp, &b, 1, h, 1, w, h, w) == 0 { "Second" } else { "First" });
}
