// BOJ 16236 [Baby Shark]
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

const DX: [i8; 4] = [-1, 0, 0, 1];
const DY: [i8; 4] = [0, -1, 1, 0];
fn bfs(map: &Vec<Vec<u8>>, i: i8, j: i8, n: usize, sz: u8)
    -> Option<(i8, i8, u16)> { // (ni, nj, d)
    let mut prey = Vec::new();
    let mut dist = vec![vec![0u16; n]; n];

    let mut q = VecDeque::new();
    q.push_back((i, j, 1u16));
    dist[i as usize][j as usize] = 1;
    loop {
        if q.is_empty() { break; }
        let mut nq = VecDeque::new();
        while let Some((i, j, d)) = q.pop_front() {
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (i + dx, j + dy);
                if nx < 0 || nx >= n as i8 || ny < 0 || ny >= n as i8 { continue; }
                if dist[nx as usize][ny as usize] != 0 { continue; }
                if map[nx as usize][ny as usize] > sz { continue; }
                if map[nx as usize][ny as usize] != 0 && map[nx as usize][ny as usize] < sz {
                    prey.push((nx, ny, d));
                } else {
                    dist[nx as usize][ny as usize] = d + 1;
                    nq.push_back((nx, ny, d + 1));
                }
            }
        }
        if !prey.is_empty() { break; }
        while let Some((i, j, d)) = nq.pop_front() {
            q.push_back((i, j, d));
        }
    }

    return if prey.is_empty() { None }
    else { prey.sort(); Some(prey[0])}
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut map = vec![vec![0u8; n]; n];
    let (mut x, mut y) = (0, 0);
    for i in 0..n { for j in 0..n {
        map[i][j] = next::<u8>(&mut it);
        if map[i][j] == 9 { x = i as i8; y = j as i8; map[i][j] = 0 }
    }}

    let (mut sz, mut cnt, mut ans) = (2, 0, 0);
    while let Some((i, j, d)) = bfs(&map, x, y, n, sz) {
        ans += d;
        map[i as usize][j as usize] = 0;
        x = i; y = j;
        cnt += 1;
        if cnt == sz { sz += 1; cnt = 0; }
    }

    println!("{}", ans);
}
