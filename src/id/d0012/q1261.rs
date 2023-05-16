// BOJ 1261 [Algospot]
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

const DX: [i8; 4] = [0, 0, -1, 1];
const DY: [i8; 4] = [-1, 1, 0, 0];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (m, n) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut map = vec![vec![false; m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            map[i][j] = c == '1';
        }
    }

    let mut dist = vec![vec![222; m]; n];
    let mut q = VecDeque::new();
    q.push_back((0i8, 0i8));
    dist[0][0] = 0;
    while let Some((x, y)) = q.pop_front() {
        if x == n as i8 - 1 && y == m as i8 - 1 { break; }
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            if x + dx < 0 || x + dx >= n as i8 || y + dy < 0 || y + dy >= m as i8 { continue; }

            let (nx, ny) = ((x + dx) as usize, (y + dy) as usize);
            if dist[nx][ny] <= dist[x as usize][y as usize] + map[nx][ny] as u8 { continue; }

            dist[nx][ny] = dist[x as usize][y as usize] + map[nx][ny] as u8;
            match map[nx][ny] {
                true => q.push_back((nx as i8, ny as i8)),
                false => q.push_front((nx as i8, ny as i8)),
            }
        }
    }
    writeln!(so, "{}", dist[n-1][m-1]).ok();
}
