// BOJ 31477 [Pigtail Rescue]
fn dfs(adj: &Vec<Vec<(usize, usize)>>, vis: &mut Vec<bool>, u: usize) -> usize {
    vis[u] = true;
    let mut ret = 0;
    for &(v, x) in &adj[u] {
        if vis[v] { continue; }
        ret += x.min(dfs(adj, vis, v));
    }
    return if ret == 0 { 0x3f3f3f3f } else { ret };
}
pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v, w) = (next::<usize>(), next::<usize>(), next::<usize>());
        adj[u].push((v, w)); adj[v].push((u, w));
    }
    let mut vis = vec![false; n+1];
    println!("{}", dfs(&adj, &mut vis, 1));
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