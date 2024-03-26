// BOJ 31502 [Don't Try This at Home]
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
    let (a, b, c) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n+1];
    let mut deg = vec![0; n+1];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(), next::<usize>(), next::<usize>());
        adj[u].push((v, w));
        adj[v].push((u, w));
        deg[u] += 1; deg[v] += 1;
    }
    let adj = adj.into_iter().map(|mut a| {
        a.sort_unstable();
        let mut b = vec![];
        if !a.is_empty() {
            b.push((a[0].0, a[0].1));
            for (v, w) in a.into_iter().skip(1) {
                if b.last().unwrap().0 == v {
                    b.last_mut().unwrap().1 = b.last().unwrap().1.min(w);
                } else {
                    b.push((v, w));
                }
            }
        }
        b.sort_unstable_by(|x, y| deg[y.0].cmp(&deg[x.0]).then(y.0.cmp(&x.0)));
        b
    }).collect::<Vec<_>>();

    let mut p = vec![0; n+1];
    let mut q = vec![b];
    'x: while !q.is_empty() {
        let mut nq = vec![];
        for u in q {
            for &(v, _) in adj[u].iter() {
                if p[v] == 0 { p[v] = u; nq.push(v); }
                if v == c { break 'x; }
            }
        }
        q = nq;
    }

    let mut path = vec![c];
    while *path.last().unwrap() != b {
        path.push(p[*path.last().unwrap()]);
    }

    let mut dist = vec![INF; n+1];
    dijkstra(&adj, &mut dist, a);
    path.sort_unstable_by(|x, y| dist[*x].cmp(&dist[*y]).then(x.cmp(y)));
    println!("{}", path[0]);
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