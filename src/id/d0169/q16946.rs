// BOJ 16946 [Break and Move 4]
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

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];
fn fill(col: &mut Vec<Vec<u32>>, vis: &mut Vec<Vec<bool>>, map: &Vec<Vec<bool>>,
        i: usize, j: usize, n: usize, m: usize, c: u32) -> u32 {
    let mut q = VecDeque::new();
    q.push_back((i, j));
    vis[i][j] = true;

    let mut cnt = 0;
    while let Some((x, y)) = q.pop_front() {
        cnt += 1;
        col[x][y] = c;
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }

            let (nx, ny) = (nx as usize, ny as usize);
            if map[nx][ny] || vis[nx][ny] { continue; }

            vis[nx][ny] = true;
            q.push_back((nx, ny));
        }
    }

    cnt
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = vec![vec![false; m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            map[i][j] = c == '1';
        }
    }

    let mut col = vec![vec![0; m]; n];
    let mut vis = vec![vec![false; m]; n];
    let mut sz = vec![0];
    let mut c = 1;
    for i in 0..n { for j in 0..m {
        if map[i][j] || vis[i][j] { continue; }
        sz.push(fill(&mut col, &mut vis, &map, i, j, n, m, c));
        c += 1;
    }}

    for i in 0..n {
        for j in 0..m {
            if map[i][j] {
                let mut v = vec![];
                let mut r = 1;
                for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                    let (nx, ny) = (i as i32 + dx, j as i32 + dy);
                    if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }

                    let (nx, ny) = (nx as usize, ny as usize);
                    if map[nx][ny] { continue; }

                    let c = col[nx][ny];
                    if v.contains(&c) { continue; }
                    v.push(c);
                    r += sz[c as usize];
                }
                write!(so, "{}", r % 10).ok();
            } else {
                write!(so, "0").ok();
            }
        }
        writeln!(so).ok();
    }
}
