// BOJ 8987 [Aquarium 3]
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
    lspan: usize, rspan: usize, vol: i64,
}
impl Node {
    fn new(h: usize, a: usize, b: usize, p: usize) -> Self {
        Self { h, a, b, p, l: 0, r: 0, lspan: 0, rspan: 0, vol: 0 }
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
    nodes[curr].vol = (rspan - lspan) as i64 * (nodes[curr].h - nodes[nodes[curr].p].h) as i64;
    (lspan, rspan)
}

fn dfs(nodes: &mut Vec<Node>, ori: &mut Vec<usize>, par: &mut Vec<usize>, ett: &mut Vec<(usize, usize)>, sums: &mut Vec<i64>, curr: usize, pidx: usize, idx: usize, vol: i64) -> usize {
    let vsum = vol + nodes[curr].vol;
    let mut nidx = idx;

    ori.push(curr);
    par.push(pidx);
    ett.push((idx, 0));
    sums.push(0);

    if nodes[curr].l > 0 {
        let i = dfs(nodes, ori, par, ett, sums,nodes[curr].l, idx, idx+1, vsum);
        nidx = i;
    }
    if nodes[curr].r > 0 {
        let i = dfs(nodes, ori, par, ett, sums,nodes[curr].r, idx, nidx+1, vsum);
        nidx = i;
    }

    ett[idx].1 = nidx;
    sums[idx] = vsum;
    nidx
}

// Maximum Segment Tree with Lazy Propagation
struct SegTree {
    n: usize,
    v: Vec<i64>,
    idx: Vec<usize>,
    lazy: Vec<i64>,
}
impl SegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, v: vec![0; m<<1], idx: vec![0; m<<1], lazy: vec![0; m<<1] }
    }
    fn build(&mut self, a: &[i64]) {
        for i in 0..a.len() {
            self.v[self.n + i] = a[i];
            self.idx[self.n + i] = i;
        }
        for i in (1..self.n).rev() {
            if self.v[i<<1] > self.v[i<<1|1] {
                self.v[i] = self.v[i<<1];
                self.idx[i] = self.idx[i<<1];
            } else {
                self.v[i] = self.v[i<<1|1];
                self.idx[i] = self.idx[i<<1|1];
            }
        }
    }
    fn propagate(&mut self, x: usize, s: usize, e: usize) {
        if self.lazy[x] == 0 { return; }
        self.v[x] += self.lazy[x];
        if s < e {
            self.lazy[x<<1] += self.lazy[x];
            self.lazy[x<<1|1] += self.lazy[x];
        }
        self.lazy[x] = 0;
    }
    fn update(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize, v: i64) {
        self.propagate(x, s, e);
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.lazy[x] += v;
            self.propagate(x, s, e);
        } else {
            let m = (s + e) >> 1;
            self.update(x<<1, s, m, l, r, v);
            self.update(x<<1|1, m+1, e, l, r, v);
            if self.v[x<<1] > self.v[x<<1|1] {
                self.v[x] = self.v[x<<1];
                self.idx[x] = self.idx[x<<1];
            } else {
                self.v[x] = self.v[x<<1|1];
                self.idx[x] = self.idx[x<<1|1];
            }
        }
    }
    fn query(&mut self, x: usize, s: usize, e: usize, l: usize, r: usize) -> (i64, usize) {
        self.propagate(x, s, e);
        if r < s || e < l { return (0, 0) }
        if l <= s && e <= r {
            (self.v[x], self.idx[x])
        } else {
            let m = (s + e) >> 1;
            self.query(x<<1, s, m, l, r).max(self.query(x<<1|1, m+1, e, l, r))
        }
    }
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
    let k = next(&mut it);

    let mut nodes = Vec::new();
    nodes.push(Node::new(0, 0, 0, 0));
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
    });

    let root = nodes[0].r;
    span(&mut nodes, root);

    let mut ori = Vec::new();
    let mut par = Vec::new();
    let mut ett = Vec::new();
    let mut sums = Vec::new();
    dfs(&mut nodes, &mut ori, &mut par, &mut ett, &mut sums, root, 0, 0, 0);

    let mut seg = SegTree::new(sums.len());
    seg.build(&sums);
    let mut vis = vec![false; n/2-1];
    let mut ans = 0;
    for _ in 0..k {
        let (val, mut idx) = seg.query(1, 0, seg.n-1, 0, seg.n-1);
        ans += val;
        while !vis[idx] {
            vis[idx] = true;
            seg.update(1, 0, seg.n-1, ett[idx].0, ett[idx].1, -nodes[ori[idx]].vol);
            idx = par[idx];
        }
    }

    writeln!(so, "{}", ans).ok();
}
