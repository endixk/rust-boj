// BOJ 31568 [The Guardian of the Forest]
fn dfs1(u: usize, p: usize, adj: &Vec<Vec<usize>>, par: &mut Vec<usize>) {
    par[u] = p;
    for &v in &adj[u] {
        if v == p { continue; }
        dfs1(v, u, adj, par);
    }
}
fn dfs2(u: usize, adj: &Vec<Vec<usize>>, pv: &Vec<bool>, k: usize, ans: &mut usize, vis: &mut Vec<bool>) {
    *ans = (*ans).max(k);
    vis[u] = true;
    for &v in &adj[u] {
        if vis[v] { continue; }
        dfs2(v, adj, pv, k + if pv[v] { 0 } else { 1 }, ans, vis);
    }
}
pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u-1].push(v-1); adj[v-1].push(u-1);
    }

    let (b, a) = (next::<usize>() - 1, next::<usize>() - 1);

    // dfs 1
    let mut par = vec![0; n];
    for &v in &adj[a] {
        dfs1(v, a, &adj, &mut par);
    }

    // reconstruct path from a to b
    let mut path = vec![b];
    while *path.last().unwrap() != a {
        path.push(par[*path.last().unwrap()]);
    }
    path.reverse();

    // mark subpath and prepare visited array
    let k = path.len() >> 1;
    let mut pv = vec![false; n];
    for i in 0..k { pv[path[i]] = true; }
    let mut vis = vec![false; n];
    vis[path[k]] = true;

    // dfs 2
    let mut ans = k;
    dfs2(path[k-1], &adj, &pv, k, &mut ans, &mut vis);

    println!("{}", ans);
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