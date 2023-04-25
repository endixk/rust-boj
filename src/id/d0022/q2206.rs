// BOJ 2206 [Break and Move]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

const MAX: usize = 1002;
const INF: i32 = 0x3f3f3f3f;

fn bfs(
    map: &[[u8; MAX]; MAX],
    n: usize, m: usize) -> i32 {

    let mut q: VecDeque<(usize, usize, bool)> = VecDeque::new();
    let mut grid_clean = [[INF; MAX]; MAX];
    let mut grid_break = [[INF; MAX]; MAX];

    q.push_back((1, 1, false));
    grid_clean[1][1] = 1;

    let dx: [i32; 4] = [0, 0, 1, -1];
    let dy: [i32; 4] = [1, -1, 0, 0];
    while let Some((x, y, br)) = q.pop_front() {
        if br {
            for i in 0..4 {
                let nx = (x as i32 + dx[i]) as usize;
                let ny = (y as i32 + dy[i]) as usize;
                if nx < 1 || nx > n || ny < 1 || ny > m { continue; }
                if map[nx][ny] == 1 { continue; }
                if grid_break[nx][ny] <= grid_break[x][y] + 1 { continue; }
                grid_break[nx][ny] = grid_break[x][y] + 1;
                q.push_back((nx, ny, true));
            }
        } else {
            for i in 0..4 {
                let nx = (x as i32 + dx[i]) as usize;
                let ny = (y as i32 + dy[i]) as usize;
                if nx < 1 || nx > n || ny < 1 || ny > m { continue; }
                if map[nx][ny] == 1 {
                    if grid_break[nx][ny] <= grid_clean[x][y] + 1 { continue; }
                    grid_break[nx][ny] = grid_clean[x][y] + 1;
                    q.push_back((nx, ny, true));
                } else {
                    if grid_clean[nx][ny] <= grid_clean[x][y] + 1 { continue; }
                    grid_clean[nx][ny] = grid_clean[x][y] + 1;
                    q.push_back((nx, ny, false));
                }
            }
        }
    }

    grid_clean[n][m].min(grid_break[n][m])
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = it.by_ref().next().unwrap().parse().unwrap();
    let m: usize = it.by_ref().next().unwrap().parse().unwrap();
    let mut map = [[0u8; MAX]; MAX];
    for i in 0..n {
        for (j, c) in it.by_ref().next().unwrap().chars().enumerate() {
            map[i+1][j+1] = c as u8 - '0' as u8;
        }
    }

    let ans = bfs(&map, n, m);
    if ans == INF {
        writeln!(so, "-1").unwrap();
    } else {
        writeln!(so, "{}", ans).unwrap();
    }
}
