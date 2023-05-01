// BOJ 8985 [Aquarium 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
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
    h: usize, a: usize, b: usize, // cargo
    p: usize, l: usize, r: usize, // tree
    lspan: usize, rspan: usize, hole: bool,
}
impl Node {
    fn new(h: usize, a: usize, b: usize, p: usize) -> Self {
        Self { h, a, b, p, l: 0, r: 0, lspan: 0, rspan: 0, hole: false }
    }
}

fn span(nodes: &mut Vec<Node>, curr: usize) -> (usize, usize) {
    let mut lspan = nodes[curr].a;
    let mut rspan = nodes[curr].b;
    if nodes[curr].l > 0 {
        let (l, _) = span(nodes, nodes[curr].l);
        lspan = l;
    }
    if nodes[curr].r > 0 {
        let (_, r) = span(nodes, nodes[curr].r);
        rspan = r;
    }
    nodes[curr].lspan = lspan;
    nodes[curr].rspan = rspan;
    (lspan, rspan)
}

fn solve(nodes: &Vec<Node>, curr: usize) -> (f64, usize, usize, bool) { // (time to leak, number of holes, volume, leak)
    let mut t: f64 = 0.0;
    let mut x: usize = 0;
    let mut v: usize = 0;
    let mut leak = false;

    if nodes[curr].l > 0 {
        let (lt, lx, lv, lk) = solve(nodes, nodes[curr].l);
        t = lt;
        x += lx;
        v += lv;
        leak |= lk;
    }
    if nodes[curr].r > 0 {
        let (rt, rx, rv, rk) = solve(nodes, nodes[curr].r);
        t = t.max(rt);
        x += rx;
        v += rv;
        leak |= rk;
    }

    if nodes[curr].hole {
        x += 1;
        leak = true;
    }
    let vol = (nodes[curr].rspan - nodes[curr].lspan) * (nodes[curr].h - nodes[nodes[curr].p].h);
    if leak {
        t += vol as f64 / x as f64;
    } else {
        v += vol;
    }

    (t, x, v, leak)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);
    let points = (0..n).map(|_| {
        let (x, y) = (next::<usize>(&mut it), next::<usize>(&mut it));
        (x, y)
    }).collect::<Vec<_>>();

    let mut nodes = Vec::new();
    nodes.push(Node::new(0, 0, 0, 0));
    let mut ap = vec![0; 500004];
    (1..n/2).for_each(|i| {
        let (a, h, b, _) = (
            points[i*2-1].0, points[i*2-1].1,
            points[i*2].0, points[i*2].1
        );
        // find parent
        let mut p = nodes.len() - 1;
        while nodes[p].h >= h {
            p = nodes[p].p;
            if p == 0 { break }
        }
        // update tree
        let mut node = Node::new(h, a, b, p);
        let prev = nodes[p].r;
        if prev > 0 {
            node.l = prev;
            nodes[prev].p = nodes.len();
        }
        nodes[p].r = nodes.len();
        nodes.push(node);
        ap[a] = nodes.len() - 1;
    });

    for _ in 0..next(&mut it) {
        let (a, _, _, _) = (
            next::<usize>(&mut it), next::<usize>(&mut it),
            next::<usize>(&mut it), next::<usize>(&mut it)
        );
        nodes[ap[a]].hole = true;
    }

    let root = nodes[0].r;
    span(&mut nodes, root);
    let sol = solve(&nodes, root);
    writeln!(so, "{:.2}\n{}", sol.0, sol.2).ok();
}
