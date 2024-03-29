// BOJ 30713 [Road Reform]
// Supported by GitHub Copilot

use std::collections::VecDeque;
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u].push((v, true));
        adj[v].push((u, false));
    }

    const INF: i32 = 0x3f3f3f3f;
    let mut a = vec![INF; n+1];
    let mut d = vec![INF; n+1];
    let mut q = VecDeque::new();
    q.push_back(1); a[1] = 0;
    while let Some(cur) = q.pop_front() {
        for &(nxt, _) in &adj[cur] {
            if a[nxt] <= a[cur] + 1 { continue; }
            a[nxt] = a[cur] + 1; q.push_back(nxt);
        }
        if d[cur] == INF {
            let mut r = VecDeque::new();
            r.push_back(cur); d[cur] = a[cur];
            while let Some(cur) = r.pop_front() {
                for &(nxt, c) in &adj[cur] {
                    if !c || d[nxt] != INF { continue; }
                    d[nxt] = d[cur]; r.push_back(nxt);
                }
            }
        }
    }

    let mut a = vec![INF; n+1];
    let mut e = vec![INF; n+1];
    let mut q = VecDeque::new();
    q.push_back(n); a[n] = 0;
    while let Some(cur) = q.pop_front() {
        for &(nxt, _) in &adj[cur] {
            if a[nxt] <= a[cur] + 1 { continue; }
            a[nxt] = a[cur] + 1; q.push_back(nxt);
        }
        if e[cur] == INF {
            let mut r = VecDeque::new();
            r.push_back(cur); e[cur] = a[cur];
            while let Some(cur) = r.pop_front() {
                for &(nxt, c) in &adj[cur] {
                    if c || e[nxt] != INF { continue; }
                    e[nxt] = e[cur]; r.push_back(nxt);
                }
            }
        }
    }

    let mut ans = INF;
    for i in 1..=n {
        ans = ans.min(d[i] + e[i]);
    }
    println!("{}", if ans == INF { -1 } else { ans });
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