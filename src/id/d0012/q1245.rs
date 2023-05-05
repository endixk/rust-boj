// BOJ 1245 [Guarding the Farm]
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

fn bfs(i: usize, j: usize, n: usize, m: usize,
       farm: &Vec<Vec<u16>>, vis: &mut Vec<Vec<bool>>) -> u16 {
    let mut q = VecDeque::new();
    q.push_back((i, j));
    let v = farm[i][j];
    let dx = [0, -1, -1, -1, 0, 1, 1, 1];
    let dy = [-1, -1, 0, 1, 1, 1, 0, -1];

    let mut ret = 0;
    while let Some((i, j)) = q.pop_front() {
        vis[i][j] = true;
        for (&x, &y) in dx.iter().zip(dy.iter()) {
            let x = i as i32 + x;
            let y = j as i32 + y;
            if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                if farm[x as usize][y as usize] == v {
                    if !vis[x as usize][y as usize]{
                        q.push_back((x as usize, y as usize));
                    }
                } else {
                    ret = ret.max(farm[x as usize][y as usize]);
                }
            }
        }
    }

    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut farm = vec![vec![0; m]; n];
    for i in 0..n { for j in 0..m {
        farm[i][j] = next::<u16>(&mut it);
    }}

    let mut ans = 0;
    let mut vis = vec![vec![false; m]; n];
    for i in 0..n { for j in 0..m {
        if !vis[i][j] {
            if bfs(i, j, n, m, &farm, &mut vis) < farm[i][j] {
                ans += 1;
            }
        }
    }}
    writeln!(so, "{}", ans).ok();
}
