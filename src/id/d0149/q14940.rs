// BOJ 14940 [Minimum Distances (Easy)]
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

const INF: u32 = 0x3f3f3f3f;
const DX: [i16; 4] = [0, 0, 1, -1];
const DY: [i16; 4] = [1, -1, 0, 0];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![false; m]; n];
    let (mut x, mut y) = (0, 0);
    for i in 0..n { for j in 0..m {
        let z = next::<u32>(&mut it);
        if z == 2 { x = i; y = j; }
        b[i][j] = z > 0;
    }}

    let mut d = vec![vec![INF; m]; n];
    let mut q = VecDeque::new();
    q.push_back((x, y));
    d[x][y] = 0;
    while let Some((x, y)) = q.pop_front() {
        for i in 0..4 {
            let nx = x as i16 + DX[i];
            let ny = y as i16 + DY[i];
            if nx < 0 || nx >= n as i16 || ny < 0 || ny >= m as i16 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if b[nx][ny] && d[nx][ny] == INF {
                d[nx][ny] = d[x][y] + 1;
                q.push_back((nx, ny));
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if b[i][j] { write!(so, "{} ", if d[i][j] == INF { -1 } else { d[i][j] as i32 }).ok(); }
            else { write!(so, "0 ").ok(); }
        }
        writeln!(so).ok();
    }
}
