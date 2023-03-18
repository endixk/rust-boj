// BOJ 1865 [Wormholes]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

const INF: i32 = 0x3f3f3f3f;
fn negative_cycle(adj: &Vec<Vec<(usize, i32)>>, size: usize) -> bool {
    let mut dst = vec![INF; size+1];
    let mut cnt = vec![0; size+1];
    let mut vis = vec![false; size+1];
    let mut inq = vec![false; size+1];
    let mut q = VecDeque::new();

    for i in 1..=size {
        if vis[i] { continue; }
        vis[i] = true;
        q.push_back(i);
        inq[i] = true;
        dst[i] = 0;

        while let Some(i) = q.pop_front() {
            inq[i] = false;
            for (j, w) in adj[i].iter() {
                if dst[*j] > dst[i] + w {
                    dst[*j] = dst[i] + w;
                    if !inq[*j] {
                        cnt[*j] += 1;
                        if cnt[*j] >= size {
                            return false;
                        }
                        q.push_back(*j);
                        inq[*j] = true;
                    }
                }
            }
        }
    }
    true
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    for _ in 0..it.by_ref().next().unwrap().parse().unwrap() {
        let v: Vec<usize> = it.by_ref().take(3).map(|x| x.parse().unwrap()).collect();
        let mut mat = vec![vec![INF; v[0]+1]; v[0]+1];

        // parse roads
        for _ in 0..v[1] {
            let x: usize = it.by_ref().next().unwrap().parse().unwrap();
            let y: usize = it.by_ref().next().unwrap().parse().unwrap();
            let w: i32 = it.by_ref().next().unwrap().parse().unwrap();
            mat[x][y] = w.min(mat[x][y]);
            mat[y][x] = w.min(mat[y][x]);
        }

        // parse wormholes
        for _ in 0..v[2] {
            let x: usize = it.by_ref().next().unwrap().parse().unwrap();
            let y: usize = it.by_ref().next().unwrap().parse().unwrap();
            let w: i32 = it.by_ref().next().unwrap().parse().unwrap();
            mat[x][y] = -w;
        }

        // convert to adjacency list
        let mut adj = vec![vec![]; v[0]+1];
        for i in 1..=v[0] {
            for j in 1..=v[0] {
                if mat[i][j] != INF {
                    adj[i].push((j, mat[i][j]));
                }
            }
        }

        // check negative cycle
        if negative_cycle(&adj, v[0]) {
            writeln!(so, "NO").unwrap();
        } else {
            writeln!(so, "YES").unwrap();
        }
    }
}
