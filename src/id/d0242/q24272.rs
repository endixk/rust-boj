// BOJ 24272 [A Good Tree has Many Root Nodes]
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

// Count segment tree
struct CountSegTree {
    n: usize,
    c: Vec<i32>,
    v: Vec<usize>,
}
impl CountSegTree {
    fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        Self { n: m, c: vec![0; m<<1], v: vec![0; m<<1] }
    }
    fn update(&mut self, i: usize, s: usize, e: usize, l: usize, r: usize, x: i32) {
        if r < s || e < l { return; }
        if l <= s && e <= r {
            self.c[i] += x;
        } else {
            let m = (s + e) >> 1;
            self.update(i<<1, s, m, l, r, x);
            self.update((i<<1)+1, m+1, e, l, r, x);
        }
        if self.c[i] > 0 {
            self.v[i] = e - s + 1;
        } else {
            if s == e { self.v[i] = 0; }
            else { self.v[i] = self.v[i<<1] + self.v[(i<<1)+1]; }
        }
    }
}

fn tour(et: &mut Vec<usize>, id: &mut Vec<usize>, x: &mut usize,
        adj: &Vec<Vec<usize>>, u: usize) {
    *x += 1;
    id[u] = *x;
    for &v in &adj[u] {
        if id[v] == 0 {
            tour(et, id, x, adj, v);
        }
    }
    et[id[u]] = *x - id[u];
}

fn update(seg: &mut CountSegTree, c: usize, n: usize, et: &[usize], dir: bool, add: bool) {
    if dir && add { // add p -> c relation
        seg.update(1, 1, n, c, c+et[c], 1);
    } else if !dir && add { // add p <- c relation
        seg.update(1, 1, n, 1, c-1, 1);
        seg.update(1, 1, n, c+et[c]+1, n, 1);
    } else if dir { // remove p -> c relation
        seg.update(1, 1, n, c, c+et[c], -1);
    } else { // remove p <- c relation
        seg.update(1, 1, n, 1, c-1, -1);
        seg.update(1, 1, n, c+et[c]+1, n, -1);
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n+1];
    let r = (1..n).map(|_| {
        let (u, d, v) = (next::<usize>(&mut it), next::<String>(&mut it), next::<usize>(&mut it));
        let (u, v) = if u < v { (u, v) } else { (v, u) };
        adj[u].push(v);
        adj[v].push(u);
        return if d == "--" { (u, v, 0) }
        else if d == "<-" { (u, v, -1) }
        else { (u, v, 1) };
    }).collect::<Vec<_>>();

    let mut et = vec![0; n+1];
    let mut id = vec![0; n+1];
    let mut x = 0;
    tour(&mut et, &mut id, &mut x, &adj,1);

    let mut seg = CountSegTree::new(n+1);
    let mut dir = vec![0; n+1];
    for (u, v, d) in r {
        let (_, v, d) = if id[u] < id[v] { (id[u], id[v], d) } else { (id[v], id[u], -d) };
        if d == 0 { continue; }
        dir[v] = d;
        update(&mut seg, v, n, &et, d > 0, true);
    }

    for _ in 0..next(&mut it) {
        let (u, d, v) = (next::<usize>(&mut it), next::<String>(&mut it), next::<usize>(&mut it));
        let d = if d == "--" { 0 } else if d == "<-" { -1 } else { 1 };
        let (_, v, d) = if id[u] < id[v] { (id[u], id[v], d) } else { (id[v], id[u], -d) };

        if dir[v] == 0 {
            if d != 0 {
                update(&mut seg, v, n, &et, d > 0, true);
            }
        }
        else if dir[v] < 0 {
            if d >= 0 { update(&mut seg, v, n, &et, false, false); }
            if d > 0 { update(&mut seg, v, n, &et, true, true); }
        } else {
            if d <= 0 { update(&mut seg, v, n, &et, true, false); }
            if d < 0 { update(&mut seg, v, n, &et, false, true); }
        }
        dir[v] = d;
        writeln!(so, "{}", n - seg.v[1]).ok();
    }
}