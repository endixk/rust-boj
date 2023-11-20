// BOJ 30508 [Puddles]
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

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
fn fill(p: &mut Vec<Vec<i32>>, b: &Vec<Vec<u8>>, i: usize, j: usize, n: usize, m: usize) {
    if p[i][j] == 0 { return; }
    let mut q = std::collections::VecDeque::new();
    p[i][j] = 0; q.push_back((i, j));
    while let Some((i, j)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 { continue; }
            let (x, y) = (x as usize, y as usize);
            if p[x][y] == 0 { continue; }
            if b[x][y] >= b[i][j] {
                p[x][y] = 0; q.push_back((x, y));
            }
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (h, w) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut b = vec![vec![0; m]; n];
    for i in 0..n { for j in 0..m {
        b[i][j] = next::<u8>(&mut it);
    }}

    let k = next::<usize>(&mut it);
    let q = (0..k).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();
    let mut p = vec![vec![1; m]; n];
    for &(i, j) in q.iter() {
        fill(&mut p, &b, i-1, j-1, n, m);
    }

    let mut s = vec![vec![0; m+1]; n+1];
    for i in 0..n { for j in 0..m {
        s[i+1][j+1] = s[i+1][j] + s[i][j+1] - s[i][j] + p[i][j];
    }}

    let mut ans = 0;
    for i in 1..=n-h+1 { for j in 1..=m-w+1 {
        let (x, y) = (i+h-1, j+w-1);
        let sum = s[x][y] - s[i-1][y] - s[x][j-1] + s[i-1][j-1];
        if sum == 0 { ans += 1; }
    }}
    writeln!(so, "{}", ans)?;

    Ok(())
}
