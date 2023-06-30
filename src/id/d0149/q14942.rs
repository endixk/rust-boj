// BOJ 14942 [Ants]
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

struct Node {
    i: usize,
    p: Vec<usize>,
    w: Vec<i32>,
}
impl Node {
    fn new(i: usize) -> Self {
        Self {
            i, p: vec![0; 18], w: vec![-1; 18],
        }
    }
}

fn build(nodes: &mut Vec<Node>, adj: &Vec<Vec<(usize, i32)>>,
         par: usize, cur: usize, w: i32) {
    nodes[cur].p[0] = par;
    nodes[cur].w[0] = w;
    for i in 0..17 {
        if nodes[nodes[cur].p[i] as usize].w[i] == -1 { break; }
        nodes[cur].p[i + 1] = nodes[nodes[cur].p[i] as usize].p[i];
        nodes[cur].w[i + 1] = nodes[cur].w[i] + nodes[nodes[cur].p[i] as usize].w[i];
    }
    for &(nxt, nw) in &adj[cur] {
        if nxt == par { continue; }
        build(nodes, adj, cur, nxt, nw);
    }
}

fn go(nodes: &Vec<Node>, cur: usize, w: i32) -> usize {
    if cur == 0 { return 0; }
    if nodes[cur].w[0] > w { return cur; }
    for i in (0..17).rev() {
        if nodes[cur].w[i] != -1 && nodes[cur].w[i] <= w {
            return go(nodes, nodes[cur].p[i], w - nodes[cur].w[i]);
        }
    }
    unreachable!();
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let (u, v, w) = (
            next::<usize>(&mut it) - 1,
            next::<usize>(&mut it) - 1,
            next::<i32>(&mut it));
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut nodes = vec![];
    (0..n).for_each(|i| nodes.push(Node::new(i)));
    build(&mut nodes, &adj, 0, 0, -1);

    for i in 0..n {
        writeln!(so, "{}", go(&nodes, i, v[i]) + 1).unwrap();
    }
}
