// BOJ 24799 [RA Duty Scheduler]
use std::collections::VecDeque;
const SRC: usize = 97;
const SNK: usize = 98;
const MAX: usize = 99;
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
    let (m, n) = (next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; MAX];
    let mut cap = vec![vec![0; MAX]; MAX];

    let mut names = vec![];
    for i in 0..m {
        adj[SRC].push(i); adj[i].push(SRC);
        names.push(next::<String>());
        for _ in 0..next() {
            let d = next::<usize>() - 1;
            adj[i].push(m + d); adj[m + d].push(i);
            cap[i][m + d] = 1;
        }
    }
    for i in 0..n {
        adj[m + i].push(SNK); adj[SNK].push(m + i);
        cap[m + i][SNK] = 2;
    }

    for x in 1.. {
        for i in 0..m { cap[SRC][i] = x; }
        let mut flo = vec![vec![0; MAX]; MAX];
        let mut lvl = vec![0; MAX];
        let mut wrk = vec![0; MAX];
        if dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, SRC, SNK) == 2 * n as i32 {
            let mut plan = vec![vec![]; n];
            for i in 0..m {
                for &j in &adj[i] {
                    if m <= j && j < m + n && flo[i][j] == 1 {
                        plan[j - m].push(names[i].clone());
                    }
                }
            }

            println!("{}", x);
            for i in 0..n {
                println!("Day {}: {}", i+1, plan[i].join(" "));
            }
            break;
        }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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