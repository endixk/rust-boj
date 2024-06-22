// BOJ 10673 [Cow Routing]
const INF: u64 = 0x3f3f3f3f3f3f3f3f;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn dijkstra(adj: &Vec<Vec<(u64, u64)>>, dist: &mut Vec<(u64, u64)>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse((0, 0)), src));
    dist[src] = (0, 0);
    while let Some((Reverse((dx, dc)), u)) = pq.pop() {
        if dist[u].0 < dx { continue; }
        for (i, &(w, c)) in adj[u].iter().enumerate() {
            let (y, z) = (dx + w, dc + c);
            if dist[i].0 > y || dist[i].0 == y && dist[i].1 > z {
                dist[i] = (y, z);
                pq.push((Reverse(dist[i]), i));
            }
        }
    }
}

pub fn main() { read();
    let (a, b, n) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![(INF, INF); 1001]; 1001];
    for _ in 0..n {
        let (c, m) = (next::<u64>(), next::<usize>());
        let v = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
        for i in 0..m-1 { for j in i+1..m {
            let (x, y) = (v[i], v[j]);
            if adj[x][y].0 > c { adj[x][y] = (c, (j - i) as u64); }
            else if adj[x][y].0 == c && adj[x][y].1 > m as u64 { adj[x][y].1 = (j - i) as u64; }
        }}
    }

    let mut dist = vec![(INF, INF); 1001];
    dijkstra(&adj, &mut dist, a);
    if dist[b].0 == INF { println!("-1 -1"); }
    else { println!("{} {}", dist[b].0, dist[b].1); }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}