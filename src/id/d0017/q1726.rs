// BOJ 1726 [Robot]
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
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut adj = vec![vec![]; r * c * 4];
    for i in 0..r*c {
        adj[i<<2].push(i<<2|1);
        adj[i<<2|1].push(i<<2);
        adj[i<<2|1].push(i<<2|2);
        adj[i<<2|2].push(i<<2|1);
        adj[i<<2|2].push(i<<2|3);
        adj[i<<2|3].push(i<<2|2);
        adj[i<<2|3].push(i<<2);
        adj[i<<2].push(i<<2|3);
    }
    let mut map = vec![vec![false; c]; r];
    for i in 0..r { for j in 0..c {
        map[i][j] = next::<i32>(&mut it) == 0;
    }}
    for i in 0..r { for j in 0..c {
        if !map[i][j] { continue; }
        let mut x = i+1;
        while x < r && x < i+4 && map[x][j] {
            adj[(i*c+j)<<2|2].push((x*c+j)<<2|2);
            adj[(x*c+j)<<2].push((i*c+j)<<2);
            x += 1;
        }
        let mut y = j+1;
        while y < c && y < j+4 && map[i][y] {
            adj[(i*c+j)<<2|3].push((i*c+y)<<2|3);
            adj[(i*c+y)<<2|1].push((i*c+j)<<2|1);
            y += 1;
        }
    }}
    let (i, j, d) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<u8>(&mut it));
    let src = match d {
        1 => (i*c+j)<<2|3,
        2 => (i*c+j)<<2|1,
        3 => (i*c+j)<<2|2,
        _ => (i*c+j)<<2,
    };
    let (i, j, d) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<u8>(&mut it));
    let dst = match d {
        1 => (i*c+j)<<2|3,
        2 => (i*c+j)<<2|1,
        3 => (i*c+j)<<2|2,
        _ => (i*c+j)<<2,
    };

    let mut q = VecDeque::new();
    let mut vis = vec![-1; r*c*4];
    q.push_back(src); vis[src] = 0;
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if vis[v] != -1 { continue; }
            vis[v] = vis[u] + 1;
            q.push_back(v);
        }
    }
    writeln!(so, "{}", vis[dst]).unwrap();
}
