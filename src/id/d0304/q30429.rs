// BOJ 30429 [Reavers]
// Supported by GitHub Copilot

fn dfs(adj: &Vec<Vec<(usize, bool)>>, vis: &mut Vec<bool>, cur: usize, p: bool) -> (usize, usize) {
    let (mut x, mut y) = (1, if p { 1 } else { 0 });
    vis[cur] = true;
    for &(nxt, c) in &adj[cur] {
        if !vis[nxt] {
            let (a, b) = dfs(adj, vis, nxt, if c { !p } else { p });
            x += a; y += b;
        }
    }
    (x, y)
}
pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n];
    for u in 0..n {
        let (v, c) = (next::<usize>()-1, next::<u8>());
        adj[u].push((v, c == 1));
        adj[v].push((u, c == 1));
    }

    let mut vis = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if !vis[i] {
            let (a, b) = dfs(&adj, &mut vis, i, true);
            ans += b.min(a - b);
        }
    }
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