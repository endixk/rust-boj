// BOJ 11493 [Coin Swap]
// Supported by GitHub Copilot

use std::collections::VecDeque;
const SRC: usize = 0;
const SNK: usize = 1001;
const MAX: usize = 1002;
const INF: i32 = 0x3f3f3f3f;
fn mcmf(adj: &Vec<Vec<usize>>, cst: &Vec<Vec<i32>>,
        flo: &mut Vec<Vec<i32>>, cap: &mut Vec<Vec<i32>>) -> i32 {
    let mut csum = 0;
    loop {
        let mut prv = vec![MAX; MAX];
        let mut dst = vec![INF; MAX];
        dst[SRC] = 0;
        prv[SRC] = SRC;

        let mut q = VecDeque::new();
        let mut inq = vec![false; MAX];
        inq[SRC] = true;
        q.push_back(SRC);

        // SPFA
        while let Some(cur) = q.pop_front() {
            inq[cur] = false;
            for &nxt in &adj[cur] {
                if cap[cur][nxt] > flo[cur][nxt] && dst[nxt] > dst[cur] + cst[cur][nxt] {
                    dst[nxt] = dst[cur] + cst[cur][nxt];
                    prv[nxt] = cur;
                    if !inq[nxt] {
                        inq[nxt] = true;
                        q.push_back(nxt);
                    }
                }
            }
        }
        if prv[SNK] == MAX { break }

        // find min flow
        let mut mflo = INF;
        let mut cur = SNK;
        while cur != SRC {
            let p = prv[cur];
            mflo = mflo.min(cap[p][cur] - flo[p][cur]);
            cur = p;
        }

        // update flow
        let mut cur = SNK;
        while cur != SRC {
            let p = prv[cur];
            csum += mflo * cst[p][cur];
            flo[p][cur] += mflo;
            flo[cur][p] -= mflo;
            cur = p;
        }
    }
    csum
}

pub fn main() { read();
    for _ in 0..next() {
        let (n, m) = (next::<usize>(), next::<usize>());
        let mut adj = vec![vec![]; MAX];
        let mut cst = vec![vec![0; MAX]; MAX];
        let mut flo = vec![vec![0; MAX]; MAX];
        let mut cap = vec![vec![0; MAX]; MAX];

        for _ in 0..m {
            let (i, j) = (next::<usize>(), next::<usize>());
            adj[i+n].push(j); adj[j].push(i+n);
            cst[i+n][j] = 1; cst[j][i+n] = -1;
            cap[i+n][j] = INF;
            adj[j+n].push(i); adj[i].push(j+n);
            cst[j+n][i] = 1; cst[i][j+n] = -1;
            cap[j+n][i] = INF;
        }
        for i in 1..=n {
            adj[i].push(i+n); adj[i+n].push(i);
            cap[i][i+n] = INF;
            if next::<u8>() == 0 {
                adj[i+n].push(SNK); adj[SNK].push(i+n);
                cap[i+n][SNK] = 1;
            }
        }
        for i in 1..=n {
            if next::<u8>() == 0 {
                adj[SRC].push(i); adj[i].push(SRC);
                cap[SRC][i] = 1;
            }
        }

        println!("{}", mcmf(&adj, &cst, &mut flo, &mut cap));
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