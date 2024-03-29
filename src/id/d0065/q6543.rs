// BOJ 6543 [The Bottom of a Graph]
fn tarjan(scc: &mut Vec<Vec<usize>>, sid: &mut Vec<usize>, adj: &Vec<Vec<usize>>, ids: &mut Vec<usize>,
          vis: &mut Vec<bool>, stk: &mut Vec<usize>, cnt: &mut usize, cur: usize) -> usize {
    *cnt += 1;
    let mut ret = *cnt;
    vis[cur] = true;
    ids[cur] = *cnt;
    stk.push(cur);

    for &nxt in &adj[cur] {
        if ids[nxt] == 0 {
            ret = ret.min(tarjan(scc, sid, adj, ids, vis, stk, cnt, nxt));
        } else if vis[nxt] {
            ret = ret.min(ids[nxt]);
        }
    }

    if ret == ids[cur] {
        let mut scc_cur = Vec::new();
        loop {
            let top = stk.pop().unwrap();
            scc_cur.push(top);
            sid[top] = scc.len();
            vis[top] = false;
            if top == cur { break; }
        }
        scc_cur.sort_unstable();
        scc.push(scc_cur);
    }

    ret
}
pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let m = next::<usize>();
        let mut adj = vec![vec![]; n + 1];
        for _ in 0..m {
            let (a, b) = (next::<usize>(), next::<usize>());
            adj[a].push(b);
        }

        let mut scc = Vec::new();
        let mut sid = vec![0; n + 1];
        let mut ids = vec![0; n + 1];
        let mut vis = vec![false; n + 1];
        let mut stk = Vec::new();
        let mut cnt = 0;
        for i in 1..=n {
            if ids[i] == 0 {
                tarjan(&mut scc, &mut sid, &adj, &mut ids, &mut vis, &mut stk, &mut cnt, i);
            }
        }

        let mut out = vec![0; scc.len()];
        let mut vis = vec![vec![false; scc.len()]; scc.len()];
        for i in 1..=n {
            for &j in &adj[i] {
                if sid[i] != sid[j] && !vis[sid[i]][sid[j]] {
                    vis[sid[i]][sid[j]] = true;
                    out[sid[i]] += 1;
                }
            }
        }

        let mut ans = Vec::new();
        for i in 1..=n {
            if out[sid[i]] == 0 {
                ans.push(i);
            }
        }
        ans.sort_unstable();
        for i in ans {
            print!("{} ", i);
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