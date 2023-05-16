// BOJ 2638 [Cheese]
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

const DX: [i32; 4] = [-1, 1, 0, 0];
const DY: [i32; 4] = [0, 0, -1, 1];
fn update(b: &mut Vec<Vec<bool>>, q: &mut VecDeque<(usize, usize)>,
          a: &Vec<Vec<bool>>, n: usize, m: usize) {
    while let Some((x, y)) = q.pop_front() {
        b[x][y] = true;
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if b[nx][ny] || a[nx][ny] { continue; }
            b[nx][ny] = true;
            q.push_back((nx, ny));
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![false; m]; n];
    let mut v = Vec::new();
    for i in 0..n { for j in 0..m {
        a[i][j] = next::<i32>(&mut it) == 1;
        if a[i][j] { v.push((i, j)); }
    }}

    let mut b = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    update(&mut b, &mut q, &a, n, m);

    let mut round = 0;
    while !v.is_empty() {
        let mut vx = Vec::new();
        let mut vy = Vec::new();
        for (i, j) in v {
            let mut c = 0;
            if b[i-1][j] { c += 1; }
            if b[i+1][j] { c += 1; }
            if b[i][j-1] { c += 1; }
            if b[i][j+1] { c += 1; }
            if c >= 2 { vx.push((i, j)); }
            else { vy.push((i, j)); }
        }

        for (i, j) in vx {
            a[i][j] = false;
            q.push_back((i, j));
        }
        update(&mut b, &mut q, &a, n, m);
        v = vy;
        round += 1;
    }

    writeln!(so, "{}", round).ok();
}
