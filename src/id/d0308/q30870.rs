// BOJ 30870 [Making Cycleless Graphs]
// Supported by GitHub Copilot

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let (x, y) = (find(root, x), find(root, y));
    if x == y { return; }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut t = vec![vec![]; n+1];
    let mut v = vec![false; n+1];
    let mut q = std::collections::VecDeque::new();
    for _ in 0..k {
        let u = next::<usize>();
        v[u] = true; q.push_back(u);
    }
    let mut i = 1;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let u = q.pop_front().unwrap();
            t[i].push(u);
            for &x in &adj[u] {
                if !v[x] { v[x] = true; q.push_back(x); }
            }
        }
        i += 1;
    }

    let mut root = (0..=n).collect::<Vec<_>>();
    let mut rank = vec![0; n+1];
    v = vec![false; n+1];
    for j in (1..i).rev() {
        for &u in &t[j] {
            v[u] = true;
            for &x in &adj[u] {
                if v[x] {
                    let (p, q) = (find(&mut root, u), find(&mut root, x));
                    if p == q { println!("{}", j); return; }
                    union(&mut root, &mut rank, p, q);
                }
            }
        }
    }
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