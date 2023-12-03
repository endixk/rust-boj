// BOJ 30879 [I Wonder What’s for Dinner...]
// Supported by GitHub Copilot

fn tarjan(
    scc: &mut Vec<Vec<usize>>,
    adj: &Vec<Vec<usize>>,
    ids: &mut Vec<usize>,
    vis: &mut Vec<bool>,
    stk: &mut Vec<usize>,
    cnt: &mut usize,
    cur: usize) -> usize {

    *cnt += 1;
    let mut ret = *cnt;
    vis[cur] = true;
    ids[cur] = *cnt;
    stk.push(cur);

    for &nxt in &adj[cur] {
        if ids[nxt] == 0 {
            ret = ret.min(tarjan(scc, adj, ids, vis, stk, cnt, nxt));
        } else if vis[nxt] {
            ret = ret.min(ids[nxt]);
        }
    }

    if ret == ids[cur] {
        let mut scc_cur = Vec::new();
        loop {
            let top = stk.pop().unwrap();
            scc_cur.push(top);
            vis[top] = false;
            if top == cur { break; }
        }
        scc_cur.sort_unstable();
        scc.push(scc_cur);
    }

    ret
}
#[inline] fn node(id: i32, n: usize) -> usize {
    match id > 0 {
        true => id as usize,
        false => (n as i32 - id) as usize
    }
}
fn sat(rel: &[(i32, i32)], n: usize) -> bool {
    let mut adj = vec![vec![]; 2*n+1];
    for &(u, v) in rel {
        adj[node(-u, n)].push(node(v, n));
        adj[node(-v, n)].push(node(u, n));
    }

    let mut scc = Vec::new();
    let mut ids = vec![0; 2*n+1];
    let mut vis = vec![false; 2*n+1];
    let mut stk = Vec::new();
    let mut cnt = 0;
    for i in 1..=2*n {
        if ids[i] == 0 {
            tarjan(&mut scc, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
        }
    }

    for cc in scc {
        for &v in &cc {
            if v > n { continue; }
            if cc.binary_search(&(v+n)).is_ok() { return false; }
        }
    }
    true
}

pub fn main() { read();
    let n = next::<usize>();
    let mut q = vec![];
    let mut rel = vec![];
    for _ in 0..n {
        if next::<u8>() == 1 {
            rel.push((next::<i32>(), next::<i32>()));
        } else {
            q.push(rel.len());
        }
    }

    let (mut l, mut r) = (0, rel.len()+1);
    while l < r {
        let m = (l+r)/2;
        if sat(&rel[..m], 111111) {
            l = m+1;
        } else {
            r = m;
        }
    }

    for &i in &q {
        println!("{}", if i < l { "YES DINNER" } else { "NO DINNER" });
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