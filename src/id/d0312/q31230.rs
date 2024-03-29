use std::collections::BinaryHeap;
use std::cmp::Reverse;

const INF: usize = 0x3f3f3f3f3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, tree: &mut Vec<Vec<usize>>, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for &(v, w) in adj[u].iter() {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                pq.push((Reverse(dist[v]), v));
                tree[v].clear();
                tree[v].push(u);
            } else if dist[v] == dist[u] + w {
                tree[v].push(u);
            }
        }
    }
}

fn dfs(tree: &Vec<Vec<usize>>, vis: &mut Vec<bool>, u: usize) {
    vis[u] = true;
    for &v in tree[u].iter() {
        if !vis[v] { dfs(tree, vis, v); }
    }
}
pub fn main() { read();
    let (n, m, a, b) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(), next::<usize>(), next::<usize>());
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    let mut dist = vec![INF; n+1];
    let mut tree = vec![vec![]; n+1];
    dijkstra(&adj, &mut tree, &mut dist, a);

    let mut vis = vec![false; n+1];
    dfs(&tree, &mut vis, b);

    println!("{}", vis.iter().filter(|&&b| b).count());
    vis.iter().enumerate().filter(|(_, &b)| b).for_each(|(i, _)| print!("{} ", i));
    println!();
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