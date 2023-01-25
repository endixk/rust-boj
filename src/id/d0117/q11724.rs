// BOJ 11724 [Connected Components]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn fill(graph: &[[bool; 1001]; 1001], visited: &mut [bool; 1001], src: usize, size: usize) {
    let mut q = VecDeque::new();
    q.push_back(src);
    visited[src] = true;

    while let Some(u) = q.pop_front() {
        for v in 1..=size {
            if graph[u][v] && !visited[v] {
                q.push_back(v);
                visited[v] = true;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = it.by_ref().next().unwrap().parse().unwrap();

    let mut graph = [[false; 1001]; 1001];
    let mut visited = [false; 1001];
    for _ in 0..it.by_ref().next().unwrap().parse().unwrap() {
        let u: usize = it.by_ref().next().unwrap().parse().unwrap();
        let v: usize = it.by_ref().next().unwrap().parse().unwrap();
        graph[u][v] = true;
        graph[v][u] = true;
    }

    let mut ans = 0;
    for i in 1..=n {
        if !visited[i] {
            fill(&graph, &mut visited, i, n);
            ans += 1;
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
