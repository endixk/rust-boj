// BOJ 2178 [Maze Traversal]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut maze = vec![vec![false; m+1]; n+1];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            maze[i+1][j+1] = c == '1';
        }
    }

    // BFS
    let mut q = VecDeque::new();
    let mut d = vec![vec![0; m+1]; n+1];
    q.push_back((1, 1));
    d[1][1] = 1;
    while let Some((i, j)) = q.pop_front() {
        if i == n && j == m {
            writeln!(so, "{}", d[i][j]).unwrap();
            break;
        }
        for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni < 1 || ni > n as i32 || nj < 1 || nj > m as i32 {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if maze[ni][nj] && d[ni][nj] == 0 {
                d[ni][nj] = d[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }
}
