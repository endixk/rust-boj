// BOJ 31476 [:blob_twintail_thinking:]
fn dfs(adj: &Vec<Vec<usize>>, i: usize, c: usize, t: usize, x: &mut usize) -> usize {
    *x += 1;
    return match adj[i].len() {
        0 => 0,
        1 => dfs(adj, adj[i][0], c, t, x) + c,
        _ => dfs(adj, adj[i][0], c+t, t, x).max(dfs(adj, adj[i][1], c+t, t, x)) + c + t
    }
}
fn rfs(adj: &Vec<Vec<usize>>, i: usize) -> usize {
    if adj[i].is_empty() { return 0; }
    if adj[i].len() == 1 { return rfs(adj, adj[i][0]) + 1; }
    return rfs(adj, adj[i][1]) + 1;
}
pub fn main() { read();
    let (d, n) = (next::<u32>(), next::<usize>());
    let (u, t) = (next::<usize>(), next::<usize>());

    let mut adj = vec![Vec::with_capacity(2); 1<<d|1];
    for i in 1..1<<d-1 {
        adj[i].push(i<<1);
        adj[i].push(i<<1|1);
    }
    for _ in 0..n {
        let (s, e) = (next::<usize>(), next::<usize>());
        let i = adj[s].iter().position(|&x| x == e).unwrap();
        adj[s].remove(i);
    }

    let mut x = 0;
    let c = dfs(&adj, 1, u, t, &mut x);
    let r = rfs(&adj, 1);
    let d = u * ((x-1) * 2 - r);
    println!(":blob_twintail_{}:", if c < d { "aww" } else if c > d { "sad" } else { "thinking" });
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