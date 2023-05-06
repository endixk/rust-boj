// BOJ 2667 [Numbering the Apartments]
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

fn dfs(map: &mut Vec<Vec<bool>>, x: usize, y: usize, n: usize) -> usize {
    let mut ret = 1;
    map[x][y] = false;
    for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        if map[nx][ny] {
            ret += dfs(map, nx, ny, n);
        }
    }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut map = vec![vec![false; n]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            map[i][j] = c == '1';
        }
    }

    let mut v = Vec::new();
    for i in 0..n { for j in 0..n {
        if map[i][j] {
            v.push(dfs(&mut map, i, j, n));
        }
    }}
    v.sort();

    writeln!(so, "{}", v.len()).unwrap();
    v.iter().for_each(|x| writeln!(so, "{}", x).unwrap());
}
