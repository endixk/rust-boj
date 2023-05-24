// BOJ 11779 [Minimum Cost Path 2]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

const INF: usize = 0x3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dst: &mut Vec<usize>, trk: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dst[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dst[u] < d { continue; }
        for (v, w) in &adj[u] {
            if dst[*v] > dst[u] + *w {
                dst[*v] = dst[u] + *w;
                trk[*v] = u;
                pq.push((Reverse(dst[*v]), *v));
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut adj = vec![vec![]; n + 1];
    for _ in 0..next(&mut it) {
        let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, w));
    }
    let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let mut dst = vec![INF; n + 1];
    let mut trk = vec![0; n + 1];
    dijkstra(&adj, &mut dst, &mut trk, u);

    let mut st = vec![];
    let mut x = v;
    while x > 0 { st.push(x); x = trk[x]; }

    writeln!(so, "{}", dst[v]).ok();
    writeln!(so, "{}", st.len()).ok();
    while let Some(x) = st.pop() {
        write!(so, "{} ", x).ok();
    }
}
