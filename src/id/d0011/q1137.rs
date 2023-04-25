// BOJ 1137 [Robot Race]
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

fn mv(board: &Vec<char>, i: usize, j: usize, n: usize, m: usize, cmd: char) -> (usize, usize) {
    let (mut ni, mut nj) = (i, j);
    match cmd {
        'N' if i > 0 && board[i*m-m+j] != '#' => { ni -= 1; },
        'S' if i < n - 1 && board[i*m+m+j] != '#' => { ni += 1; },
        'W' if j > 0 && board[i*m+j-1] != '#' => { nj -= 1; },
        'E' if j < m - 1 && board[i*m+j+1] != '#' => { nj += 1; },
        _ => {}
    }
    (ni, nj)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut board = vec!['#'; n*m];
    let (mut xi, mut xj, mut yi, mut yj, mut fi, mut fj) = (0, 0, 0, 0, 0, 0);
    for i in 0..n {
        let t = next::<String>(&mut it);
        for (j, c) in t.chars().enumerate() {
            board[i*m+j] = c;
            match c {
                'X' => {xi = i; xj = j},
                'Y' => {yi = i; yj = j},
                'F' => {fi = i; fj = j},
                _ => {}
            }
        }
    }

    const INF: usize = 0x3f3f3f3f;
    let cmd = next::<String>(&mut it);
    let mut dp = vec![INF; n*m];
    let mut ans = INF;
    for (k, c) in cmd.chars().rev().enumerate() {
        let idx = cmd.len() - k - 1;
        dp[xi*m+xj] = idx;

        let mut tp = vec![INF; n*m];
        for i in 0..n {
            for j in 0..m {
                let (ni, nj) = mv(&board, i, j, n, m, c);
                tp[i*m+j] = dp[i*m+j].min(dp[ni*m+nj]);
            }
        }

        dp = tp;
        if dp[yi*m+yj] < dp[fi*m+fj] { ans = idx; }
    }

    writeln!(so, "{}", match ans { INF => -1, _ => ans as i32 }).unwrap();
}
