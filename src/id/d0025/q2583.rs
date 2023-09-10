// BOJ 2583 [Areas]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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
fn fill(b: &mut Vec<Vec<bool>>, i: usize, j: usize, m: usize, n: usize) -> usize {
    let mut q = VecDeque::new();
    let mut cnt = 1;
    q.push_back((i, j));
    b[i][j] = false;
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if !b[nx][ny] { continue; }
            b[nx][ny] = false;
            cnt += 1;
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

    let (m, n, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![true; n]; m];
    for _ in 0..k {
        let (x1, y1) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (x2, y2) = (next::<usize>(&mut it), next::<usize>(&mut it));
        for i in y1..y2 { for j in x1..x2 { b[i][j] = false; }}
    }

    let mut v = Vec::new();
    for i in 0..m { for j in 0..n {
        if !b[i][j] { continue; }
        v.push(fill(&mut b, i, j, m, n));
    }}
    v.sort_unstable();
    writeln!(so, "{}", v.len()).unwrap();
    for i in v { write!(so, "{} ", i).unwrap(); }
}
