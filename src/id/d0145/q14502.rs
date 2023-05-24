// BOJ 14502 [Laboratory]
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
fn safe(lab: &Vec<u8>, vis: &mut Vec<bool>, n: usize, m: usize) -> u8 {
    let mut q = VecDeque::new();
    for i in 0..n { for j in 0..m {
        if lab[i*m+j] == 2 {
            q.push_back((i, j));
            vis[i*m+j] = true;
        } else if lab[i*m+j] == 1{
            vis[i*m+j] = true;
        }
    }}
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i8 + dx, y as i8 + dy);
            if nx < 0 || nx >= n as i8 || ny < 0 || ny >= m as i8 { continue; }

            let (nx, ny) = (nx as usize, ny as usize);
            if vis[nx*m+ny] || lab[nx*m+ny] != 0 { continue; }

            q.push_back((nx, ny));
            vis[nx*m+ny] = true;
        }
    }

    vis.iter().filter(|&&v| !v).count() as u8
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut lab = vec![0; m*n];
    let mut locs = Vec::new();
    for i in 0..n { for j in 0..m {
        lab[i*m+j] = next::<u8>(&mut it);
        if lab[i*m+j] == 0 { locs.push((i, j)); }
    }}

    let mut ans = 0;
    let mut vis = vec![false; m*n];
    for i in 0..locs.len()-2 {
        lab[locs[i].0*m+locs[i].1] = 1;
        for j in i+1..locs.len() -1 {
            lab[locs[j].0*m+locs[j].1] = 1;
            for k in j+1..locs.len() {
                lab[locs[k].0*m+locs[k].1] = 1;
                vis.fill(false);
                ans = ans.max(safe(&lab, &mut vis, n, m));
                lab[locs[k].0*m+locs[k].1] = 0;
            }
            lab[locs[j].0*m+locs[j].1] = 0;
        }
        lab[locs[i].0*m+locs[i].1] = 0;
    }
    writeln!(so, "{}", ans).ok();
}
