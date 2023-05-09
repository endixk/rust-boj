// BOJ 10026 [Red-Green Color-Weak]
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

fn dfs(board: &mut Vec<Vec<i16>>, i: usize, j: usize, n: usize, c: i16) {
    board[i][j] = -1;
    for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (ni, nj) = (i as i32 + dx, j as i32 + dy);
        if ni < 0 || ni >= n as i32 || nj < 0 || nj >= n as i32 {
            continue;
        }
        let (ni, nj) = (ni as usize, nj as usize);
        if board[ni][nj] == c {
            dfs(board, ni, nj, n, c);
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut board = vec![vec![0i16; n]; n];
    let mut bweak = vec![vec![0i16; n]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            board[i][j] = match c {
                'R' => 0, 'G' => 1, _ => 2,
            };
            bweak[i][j] = match c {
                'R' => 0, 'G' => 0, _ => 1,
            };
        }
    }

    let (mut bc, mut wc) = (0, 0);
    for i in 0..n { for j in 0..n {
        let (c, w) = (board[i][j], bweak[i][j]);
        if c >= 0 {
            bc += 1;
            dfs(&mut board, i, j, n, c);
        }
        if w >= 0 {
            wc += 1;
            dfs(&mut bweak, i, j, n, w);
        }
    }}
    writeln!(so, "{} {}", bc, wc).ok();
}
