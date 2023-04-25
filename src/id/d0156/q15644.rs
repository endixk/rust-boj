// BOJ 15644 [Marble Escape 3]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

fn locate(board: &Vec<Vec<char>>, i: usize, j: usize, dir: usize) -> Option<(usize, usize)> {
    // 0: up, 1: right, 2: down, 3: left
    let (mut ni, mut nj) = (i, j);
    while board[ni][nj] != '#' && board[ni][nj] != 'O' {
        ni = match dir { 0 => ni - 1, 2 => ni + 1, _ => ni };
        nj = match dir { 1 => nj + 1, 3 => nj - 1, _ => nj };
    }
    if board[ni][nj] == '#' {
        return Some((
            match dir { 0 => ni + 1, 2 => ni - 1, _ => ni },
            match dir { 1 => nj - 1, 3 => nj + 1, _ => nj }
        ));
    }
    None
}

fn go(board: &mut Vec<Vec<char>>, rx: usize, ry: usize, bx: usize, by: usize, dir: usize)
      -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let red_first = match dir {
        0 => rx < bx,
        1 => ry > by,
        2 => rx > bx,
        _ => ry < by,
    };

    if red_first {
        if let Some((rx, ry)) = locate(board, rx, ry, dir) {
            board[rx][ry] = '#';
            let blue = locate(board, bx, by, dir);
            board[rx][ry] = '.';
            (Some((rx, ry)), blue)
        } else {
            (None, locate(board, bx, by, dir))
        }
    } else {
        if let Some((bx, by)) = locate(board, bx, by, dir) {
            board[bx][by] = '#';
            let red = locate(board, rx, ry, dir);
            board[bx][by] = '.';
            (red, Some((bx, by)))
        } else {
            (locate(board, rx, ry, dir), None)
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m): (usize, usize) = (next(&mut it), next(&mut it));

    let mut board = vec![vec!['.'; m]; n];
    let (mut r, mut b) = (0, 0);
    for i in 0..n {
        let str: String = next(&mut it);
        for (j, c) in str.chars().enumerate() {
            board[i][j] = c;
            if board[i][j] == 'R' {
                r = i * m + j;
                board[i][j] = '.';
            } else if board[i][j] == 'B' {
                b = i * m + j;
                board[i][j] = '.';
            }
        }
    }

    let mut vis = vec![false; n * m * n * m];
    let mut q = VecDeque::new();
    q.push_back((r * n * m + b, 0, String::new()));
    vis[r * n * m + b] = true;
    while let Some((p, d, s)) = q.pop_front() {
        if d > 9 { break; }
        let (r, b) = (p / (n * m), p % (n * m));
        let (rx, ry) = (r / m, r % m);
        let (bx, by) = (b / m, b % m);

        for dir in 0..4 {
            let (nr, nb) = go(&mut board, rx, ry, bx, by, dir);
            if let None = nb {
                continue;
            } else if let None = nr {
                writeln!(so, "{}", d + 1).unwrap();
                writeln!(so, "{}", s + match dir { 0 => "U", 1 => "R", 2 => "D", _ => "L" }).unwrap();
                return;
            } else {
                let (nr, nb) = (nr.unwrap(), nb.unwrap());
                let np = nr.0 * m + nr.1;
                let nb = nb.0 * m + nb.1;
                if !vis[np * n * m + nb] {
                    vis[np * n * m + nb] = true;
                    q.push_back((np * n * m + nb, d + 1, s.clone() +
                        match dir { 0 => "U", 1 => "R", 2 => "D", _ => "L" }));
                }
            }
        }
    }

    writeln!(so, "-1").unwrap();
}
