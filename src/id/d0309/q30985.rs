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
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n*2+1];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(), next::<usize>(), next::<usize>());
        adj[u].push((v, w)); adj[v].push((u, w));
        adj[u+n].push((v+n, w)); adj[v+n].push((u+n, w));
    }
    for i in 1..=n {
        let e = next::<i64>();
        if e < 0 { continue; }
        adj[i].push((i+n, (k-1) * e as usize));
    }

    let mut dist = vec![INF; n*2+1];
    dijkstra(&adj, &mut dist, 1);
    if dist[n*2] == INF { println!("-1"); }
    else { println!("{}", dist[n*2]); }
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