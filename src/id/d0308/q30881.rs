// BOJ 30881 [Typographs]
// Supported by GitHub Copilot

use std::collections::BinaryHeap;
use std::cmp::Reverse;

const INF: usize = 1e18 as usize + 1;
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for &(v, w) in &adj[u] {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                pq.push((Reverse(dist[v]), v));
            }
        }
    }
}
pub fn main() { read();
    let t = next::<usize>();
    let mut f = vec![0; t+1];
    let mut r = vec![0; t+1];
    for i in 1..=t {
        let (n, m) = (next::<usize>(), next::<usize>());
        let mut adj = vec![vec![]; n+1];
        for _ in 0..m {
            let (u, v, w) = (next::<usize>(), next::<usize>(), next::<i64>());
            if w >= 0 {
                adj[u].push((v, w as usize));
            } else {
                let w = (-w) as usize;
                let (wf, wr) = (f[w], r[w]);
                if wf != INF { adj[u].push((v, wf)); }
                if wr != INF { adj[v].push((u, wr)); }
            }
        }

        let mut dst = vec![INF; n+1];
        dijkstra(&adj, &mut dst, 1);
        f[i] = dst[2];
        dst = vec![INF; n+1];
        dijkstra(&adj, &mut dst, 2);
        r[i] = dst[1];
    }
    if f[t] >= INF { println!("-1"); }
    else { println!("{}", f[t]); }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}