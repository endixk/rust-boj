// BOJ 30974 [What's your ETA?]

fn sieve(n: usize) -> Vec<bool> {
    let mut p = vec![true; n+1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if !p[i] { continue; }
        for j in (2*i..=n).step_by(i) {
            p[j] = false;
        }
    }
    p
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
const INF: usize = 0x3f3f3f3f3f3f3f3f;
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
    let (n, m) = (next::<usize>(), next::<usize>());
    let d = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let p = sieve(10_000_000);

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>()-1, next::<usize>()-1, next::<usize>());
        if p[d[u]+d[v]] {
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
    }

    let mut dist = vec![INF; n];
    dijkstra(&adj, &mut dist, 0);
    if dist[n-1] == INF { println!("Now where are you?"); }
    else { println!("{}", dist[n-1]); }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}