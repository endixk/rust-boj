// BOJ 21736 [Friend Wanted]
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

const DX: [i16; 4] = [0, 0, 1, -1];
const DY: [i16; 4] = [1, -1, 0, 0];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let b = (0..n).map(|_| {
        next::<String>(&mut it).chars().map(|c| c).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let (mut x, mut y) = (0, 0);
    for i in 0..n { for j in 0..m {
        if b[i][j] == 'I' { x = i; y = j; }
    }}

    let mut v = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    q.push_back((x, y));
    v[x][y] = true;
    let mut ans = 0;
    while let Some((x, y)) = q.pop_front() {
        for i in 0..4 {
            let nx = x as i16 + DX[i];
            let ny = y as i16 + DY[i];
            if nx < 0 || nx >= n as i16 || ny < 0 || ny >= m as i16 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if b[nx][ny] != 'X' && !v[nx][ny] {
                v[nx][ny] = true;
                if b[nx][ny] == 'P' { ans += 1; }
                q.push_back((nx, ny));
            }
        }
    }

    println!("{}", if ans == 0 { "TT".to_string() } else { ans.to_string() });
}
