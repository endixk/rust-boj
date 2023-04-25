// BOJ 2606 [Computer Virus]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn bfs(graph: &Vec<Vec<usize>>, vis: &mut Vec<bool>, src: usize) {
    let mut q = VecDeque::new();
    q.push_back(src);
    vis[src] = true;

    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if !vis[v] {
                q.push_back(v);
                vis[v] = true;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let v: Vec<usize> = it.by_ref().take(2).map(|x| x.parse().unwrap()).collect();
    let mut graph = vec![vec![]; v[0] + 1];
    let mut vis = vec![false; v[0] + 1];
    for _ in 0..v[1] {
        let w: Vec<usize> = it.by_ref().take(2).map(|x| x.parse().unwrap()).collect();
        graph[w[0]].push(w[1]);
        graph[w[1]].push(w[0]);
    }

    bfs(&graph, &mut vis, 1);
    writeln!(so, "{}", vis.iter().filter(|&&x| x).count() - 1).unwrap();
}
