// BOJ 7576 [Tomatoes]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn bfs(grid: &mut Vec<Vec<i32>>, q: &mut VecDeque<(i32, i32)>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut ret = 0;

    let vdx: Vec<i32> = vec![-1, 1, 0, 0];
    let vdy: Vec<i32> = vec![0, 0, -1, 1];
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in vdx.iter().zip(vdy.iter()) {
            if x + dx < 0 || x + dx >= n as i32 || y + dy < 0 || y + dy >= m as i32 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;
            if grid[nx as usize][ny as usize] == 0 {
                grid[nx as usize][ny as usize] = grid[x as usize][y as usize] + 1;
                ret = ret.max(grid[nx as usize][ny as usize]);
                q.push_back((nx, ny));
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

    let v = it.by_ref().take(2)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut grid = vec![vec![0; v[0]]; v[1]];
    let mut q = VecDeque::new();
    for i in 0..v[1] {
        for j in 0..v[0] {
            grid[i][j] = it.by_ref().next().unwrap().parse().unwrap();
            if grid[i][j] == 1 {
                q.push_back((i as i32, j as i32));
            }
        }
    }

    let res = bfs(&mut grid, &mut q) - 1;
    for row in grid.iter() {
        if row.iter().any(|&x| x == 0) {
            writeln!(so, "-1").unwrap();
            return;
        }
    }
    writeln!(so, "{}", match res < 0 {
        true => 0,
        false => res,
    }).unwrap();
}
