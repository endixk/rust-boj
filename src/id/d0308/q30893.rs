// BOJ 30893 [Tree Game]
// Supported by GitHub Copilot

fn dfs(p: &mut Vec<usize>, adj: &Vec<Vec<usize>>, cur: usize, par: usize, dst: usize) {
    p[cur] = par;
    if cur == dst { return; }
    for &nxt in &adj[cur] {
        if nxt != par {
            dfs(p, adj, nxt, cur, dst);
        }
    }
}
pub fn main() { read();
    let (n, s, e) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u].push(v); adj[v].push(u);
    }

    let mut p = vec![0; n+1];
    dfs(&mut p, &adj, e, 0, s);

    let mut cur = p[s];
    let mut flag = true;
    while cur != e {
        if flag && adj[cur].len() > 2 { println!("Second"); return; }
        flag = !flag;
        cur = p[cur];
    }
    println!("First");
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