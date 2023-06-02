// BOJ 14442 [Break and Move 2]
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
fn bfs(map: &[bool; 1<<20], n: usize, m: usize, k: usize) -> i32 {
    let mut q = VecDeque::new();
    let mut vis = [false; 1<<24];
    q.push_back(0);
    vis[0] = true;
    let mut d = 0;

    while !q.is_empty() {
        d += 1;
        for _ in 0..q.len() {
            let v = q.pop_front().unwrap();
            let (x, y, b) = (v >> 14, v >> 4 & 0x3ff, v & 0xf);
            if x == n-1 && y == m-1 { return d; }
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }

                let (nx, ny) = (nx as usize, ny as usize);
                let nk = if map[nx << 10 | ny] { b + 1 } else { b };
                if nk > k { continue; }

                let nxt = nx << 14 | ny << 4 | nk;
                if vis[nxt] { continue; }

                vis[nxt] = true;
                q.push_back(nxt);
            }
        }
    }

    return -1
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = [false; 1<<20];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            map[i<<10|j] = c == '1';
        }
    }

    println!("{}", bfs(&map, n, m, k));
}
