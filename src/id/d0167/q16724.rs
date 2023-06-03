// BOJ 16724 [Pied Piper]
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
    let mut map = vec![vec![(0, 0); m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            map[i][j] = match c {
                'U' => (i - 1, j),
                'D' => (i + 1, j),
                'L' => (i, j - 1),
                'R' => (i, j + 1),
                _ => (i, j),
            };
        }
    }

    let mut vis = vec![vec![false; m]; n];
    let mut prc = vec![vec![false; m]; n];
    let mut ans = 0;
    for i in 0..n { for j in 0..m {
        if vis[i][j] { continue; }
        let mut trk = Vec::new();
        let (mut x, mut y) = (i, j);
        while !prc[x][y] {
            prc[x][y] = true;
            trk.push((x, y));
            (x, y) = map[x][y];
        }

        if !vis[x][y] { ans += 1 }
        for &(x, y) in &trk { vis[x][y] = true; }
    }}

    println!("{}", ans);
}
