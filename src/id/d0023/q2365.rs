// BOJ 2365 [Number Tiles]
// Supported by GitHub Copilot

use std::collections::VecDeque;
const MAX: usize = 102;
const INF: i32 = 0x3f3f3f3f;
fn dinic_bfs(lvl: &mut Vec<i32>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, flo: &Vec<Vec<i32>>,
             src: usize, snk: usize) -> bool {
    lvl.iter_mut().for_each(|x| *x = -1);
    lvl[src] = 0;
    let mut q = VecDeque::new();
    q.push_back(src);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if lvl[v] == -1 && cap[u][v] - flo[u][v] > 0 {
                lvl[v] = lvl[u] + 1;
                q.push_back(v);
            }
        }
    }
    lvl[snk] != -1
}
fn dinic_dfs(flo: &mut Vec<Vec<i32>>, wrk: &mut Vec<usize>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, lvl: &Vec<i32>,
             cur: usize, snk: usize, f: i32) -> Option<i32> {
    if cur == snk { return Some(f); }
    while wrk[cur] < adj[cur].len() {
        let nxt = adj[cur][wrk[cur]];
        if lvl[nxt] == lvl[cur] + 1 && cap[cur][nxt] - flo[cur][nxt] > 0 {
            if let Some(df) = dinic_dfs(flo, wrk, adj, cap, lvl, nxt, snk, f.min(cap[cur][nxt] - flo[cur][nxt])) {
                flo[cur][nxt] += df;
                flo[nxt][cur] -= df;
                return Some(df);
            }
        }
        wrk[cur] += 1;
    }
    None
}
fn dinic(flo: &mut Vec<Vec<i32>>, lvl: &mut Vec<i32>, wrk: &mut Vec<usize>,
         adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, src: usize, snk: usize) -> i32 {
    let mut ret = 0;
    while dinic_bfs(lvl, adj, cap, flo, src, snk) {
        wrk.iter_mut().for_each(|x| *x = 0);
        while let Some(f) = dinic_dfs(flo, wrk, adj, cap, lvl, src, snk, INF) {
            ret += f;
        }
    }
    ret
}

pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];
    let mut flo = vec![vec![0; MAX]; MAX];
    let mut lvl = vec![0; MAX];
    let mut wrk = vec![0; MAX];
    let (src, snk) = (0, 101);

    for i in 1..=n {
        let x = next::<i32>();
        adj[src].push(i);
        adj[i].push(src);
        cap[src][i] = x;
    }
    for i in n+1..=2*n {
        let x = next::<i32>();
        adj[i].push(snk);
        adj[snk].push(i);
        cap[i][snk] = x;
    }

    for i in 1..=n {
        for j in n+1..=2*n {
            adj[i].push(j);
            adj[j].push(i);
            cap[i][j] = INF;
        }
    }
    let f = dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, src, snk);

    let (mut l, mut r) = (1, 10000);
    while l < r {
        let m = (l + r) / 2;
        for i in 1..=n {
            for j in n+1..=2*n {
                cap[i][j] = m;
            }
        }
        flo = vec![vec![0; MAX]; MAX];
        lvl = vec![0; MAX];
        wrk = vec![0; MAX];
        let g = dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, src, snk);
        if g == f { r = m; }
        else { l = m + 1; }
    }

    flo = vec![vec![0; MAX]; MAX];
    lvl = vec![0; MAX];
    wrk = vec![0; MAX];
    for i in 1..=n {
        for j in n+1..=2*n {
            cap[i][j] = l;
        }
    }
    dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, src, snk);

    println!("{}", l);
    for i in 1..=n {
        for j in n+1..=2*n {
            print!("{} ", flo[i][j]);
        }
        println!();
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}