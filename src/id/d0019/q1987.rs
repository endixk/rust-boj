// BOJ 1987 [Letters]
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

fn dfs(a: &Vec<Vec<u8>>, mask: u32, i: usize, j: usize, r: usize, c: usize) -> u8 {
    let mut ret = 1;
    if i > 0 && (mask & (1 << a[i - 1][j])) == 0 {
        ret = ret.max(dfs(a, mask | (1 << a[i - 1][j]), i - 1, j, r, c) + 1);
    }
    if i < r - 1 && (mask & (1 << a[i + 1][j])) == 0 {
        ret = ret.max(dfs(a, mask | (1 << a[i + 1][j]), i + 1, j, r, c) + 1);
    }
    if j > 0 && (mask & (1 << a[i][j - 1])) == 0 {
        ret = ret.max(dfs(a, mask | (1 << a[i][j - 1]), i, j - 1, r, c) + 1);
    }
    if j < c - 1 && (mask & (1 << a[i][j + 1])) == 0 {
        ret = ret.max(dfs(a, mask | (1 << a[i][j + 1]), i, j + 1, r, c) + 1);
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; c]; r];
    for i in 0..r {
        next::<String>(&mut it).as_bytes().iter().enumerate().for_each(|(j, &b)| a[i][j] = b - b'A');
    }
    writeln!(so, "{}", dfs(&a, 1 << a[0][0], 0, 0, r, c)).unwrap();
}
