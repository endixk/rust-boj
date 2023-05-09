// BOJ 7569 [Tomatoes 3D]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (m, n, h) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![vec![0; m]; n]; h];
    let mut q = VecDeque::new();

    for i in 0..h { for j in 0..n { for k in 0..m {
        a[i][j][k] = next::<i32>(&mut it);
        if a[i][j][k] == 1 { q.push_back((i, j, k)); }
    }}}

    let (dx, dy, dz) = (
        [0, 0, 0, 0, 1, -1],
        [0, 0, 1, -1, 0, 0],
        [1, -1, 0, 0, 0, 0]
    );
    while let Some((z, y, x)) = q.pop_front() {
        for i in 0..6 {
            let (nz, ny, nx) = (z as i32 + dz[i], y as i32 + dy[i], x as i32 + dx[i]);
            if nz < 0 || nz >= h as i32 || ny < 0 || ny >= n as i32 || nx < 0 || nx >= m as i32 { continue; }
            let (nz, ny, nx) = (nz as usize, ny as usize, nx as usize);
            if a[nz][ny][nx] == 0 {
                a[nz][ny][nx] = a[z][y][x] + 1;
                q.push_back((nz, ny, nx));
            }
        }
    }

    let mut ans = 0;
    for i in 0..h { for j in 0..n { for k in 0..m {
        if a[i][j][k] == 0 {
            println!("-1");
            return;
        }
        ans = ans.max(a[i][j][k]);
    }}}
    println!("{}", ans - 1);
}
