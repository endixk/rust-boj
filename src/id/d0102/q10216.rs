// BOJ 10216 [Count Circle Groups]
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

struct Tower {
    x: i32,
    y: i32,
    r: i32,
}
impl Tower {
    fn overlap(&self, other: &Tower) -> bool {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) <= (self.r + other.r).pow(2)
    }
}

fn dfs(v: &Vec<Tower>, vis: &mut Vec<bool>, i: usize) {
    vis[i] = true;
    for j in 0..v.len() {
        if !vis[j] && v[i].overlap(&v[j]) {
            dfs(v, vis, j);
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let v = (0..n).map(|_| {
            Tower {
                x: next(&mut it),
                y: next(&mut it),
                r: next(&mut it),
            }
        }).collect::<Vec<_>>();

        let mut cnt = 0;
        let mut vis = vec![false; n];
        for i in 0..n {
            if !vis[i] {
                dfs(&v, &mut vis, i);
                cnt += 1;
            }
        }
        writeln!(so, "{}", cnt).ok();
    }
}
