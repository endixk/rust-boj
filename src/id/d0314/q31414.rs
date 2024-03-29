// BOJ 31414 [Stationing]
fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, d: &mut Vec<usize>, cyc: &mut usize) -> usize {
    let mut ret = 1;
    for &v in &adj[u] {
        if v == p { continue; }
        if d[v] == 1 {
            d[v] = d[u] + 1;
            ret += dfs(adj, v, u, d, cyc);
        } else if d[v] < d[u] {
            *cyc = d[u] - d[v] + 1;
        }
    }
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n];
    for u in 0..n {
        let v = next::<usize>() - 1;
        adj[u].push(v); adj[v].push(u);
    }

    let mut d = vec![1; n];
    let mut ans = 0;
    for u in 0..n {
        if d[u] == 1 {
            let mut cyc = 1;
            let cnt = dfs(&adj, u, n, &mut d, &mut cyc);
            ans += cnt - if cyc & 1 == 1 { 1 } else { 0 };
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