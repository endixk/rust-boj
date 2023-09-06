// BOJ 2589 [Treasure Island]
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

use std::collections::VecDeque;
const DX: [i32; 4] = [0, 0, -1, 1];
const DY: [i32; 4] = [-1, 1, 0, 0];
fn bfs(map: &Vec<Vec<bool>>, r: usize, c: usize, i: usize, j: usize) -> usize {
    let mut q = VecDeque::new();
    let mut vis = vec![vec![false; c]; r];
    let mut d = 0;
    q.push_back((i, j));
    vis[i][j] = true;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let (x, y) = q.pop_front().unwrap();
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= r as i32 || ny < 0 || ny >= c as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if map[nx][ny] && !vis[nx][ny] {
                    q.push_back((nx, ny));
                    vis[nx][ny] = true;
                }
            }
        }
        d += 1;
    }
    d
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = vec![vec![false; c]; r];
    for i in 0..r {
        let s = next::<String>(&mut it);
        for (j, x) in s.chars().enumerate() {
            map[i][j] = x == 'L';
        }
    }

    let mut ans = 0;
    for i in 0..r {
        for j in 0..c {
            if map[i][j] {
                ans = ans.max(bfs(&map, r, c, i, j) - 1);
            }
        }
    }
    println!("{}", ans);
}
