// BOJ 14218 [Graph Traversal 2]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 == 0 { *p = p.offset(1) }
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    n
}}
fn bfs(adj: &Vec<Vec<usize>>, vis: &mut Vec<u16>, src: usize) {
    let mut q = vec![src];
    while !q.is_empty() {
        let mut nq = vec![];
        for u in q {
            for &v in &adj[u] {
                if vis[v] > vis[u] + 1 {
                    vis[v] = vis[u] + 1;
                    nq.push(v);
                }
            }
        }
        q = nq;
    }
}
pub fn main() {
    let mut p = input(555555);
    let (n, m) = (ptr(&mut p) as usize, ptr(&mut p) as usize);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = (ptr(&mut p) as usize - 1, ptr(&mut p) as usize - 1);
        adj[u].push(v); adj[v].push(u);
    }

    let mut vis = vec![9999; n];
    vis[0] = 0;
    bfs(&adj, &mut vis, 0);
    for _ in 0..ptr(&mut p) {
        let (u, v) = (ptr(&mut p) as usize - 1, ptr(&mut p) as usize - 1);
        adj[u].push(v); adj[v].push(u);
        if vis[u] + 1 < vis[v] {
            vis[v] = vis[u] + 1; bfs(&adj, &mut vis, v);
        } else if vis[v] + 1 < vis[u] {
            vis[u] = vis[v] + 1; bfs(&adj, &mut vis, u);
        };

        for i in 0..n {
            if vis[i] == 9999 { print!("-1 "); }
            else { print!("{} ", vis[i]); }
        }
        println!();
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}