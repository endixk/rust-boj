// BOJ 2424 [Treasures and Vikings]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

const DX: [i32; 4] = [0, 0, -1, 1];
const DY: [i32; 4] = [-1, 1, 0, 0];
const INF: i32 = 0x3f3f3f3f;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut y, mut v, mut t) = ((0, 0), (0, 0), (0, 0));
    let mut sea = vec![vec![false; m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            sea[i][j] = c != 'I';
            match c {
                'Y' => y = (i, j),
                'V' => v = (i, j),
                'T' => t = (i, j),
                _ => (),
            }
        }
    }

    // define reach of viking
    let mut vmap = vec![vec![INF; m]; n];
    let mut q = VecDeque::new();
    q.push_back(v);
    vmap[v.0][v.1] = 0;
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if !sea[nx][ny] || vmap[nx][ny] != INF { continue; }
            vmap[nx][ny] = vmap[x][y] + 1;
            q.push_back((nx, ny));
        }
    }

    // define horizontal sight of viking
    let mut hsight = vec![vec![INF; m]; n];
    for i in 0..n {
        let (mut l, mut r) = (0, 0);
        while l < m {
            let mut h = INF;
            while r < m && !sea[i][r] { r += 1; }
            while r < m && sea[i][r] {
                h = h.min(vmap[i][r]);
                r += 1;
            }
            while l < r {
                if sea[i][l] { hsight[i][l] = h; }
                l += 1;
            }
        }
    }

    // define vertical sight of viking
    let mut vsight = vec![vec![INF; m]; n];
    for j in 0..m {
        let (mut u, mut d) = (0, 0);
        while u < n {
            let mut h = INF;
            while d < n && !sea[d][j] { d += 1; }
            while d < n && sea[d][j] {
                h = h.min(vmap[d][j]);
                d += 1;
            }
            while u < d {
                if sea[u][j] { vsight[u][j] = h; }
                u += 1;
            }
        }
    }

    let sight = (0..n).map(|i|
        (0..m).map(|j|
            hsight[i][j].min(vsight[i][j])
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    // define reach of treasure
    let mut q = VecDeque::new();
    let mut vis = vec![vec![false; m]; n];
    q.push_back(y);
    vis[y.0][y.1] = true;
    let mut d = 1;
    while !q.is_empty() {
        let cnt = q.len();
        for _ in 0..cnt {
            let (x, y) = q.pop_front().unwrap();
            if (x, y) == t {
                println!("YES");
                return;
            }
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if !sea[nx][ny] || vis[nx][ny] || sight[nx][ny] <= d { continue; }
                vis[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
        d += 1;
    }
    println!("NO");
}
