// BOJ 6561 [Decode the Tree]
fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, f: bool) {
    if f { print!(" "); }
    print!("({}", u + 1);
    for &v in &adj[u] {
        if v == p { continue; }
        dfs(adj, v, u, true);
    }
    print!(")");
}
pub fn main() { read();
    while peek() {
        let v = next::<String>().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect::<Vec<_>>();
        if v.len() == 0 {
            if !peek() { break; }
            println!("(1)"); continue;
        }
        let n = v.len() + 1;

        let mut pru = vec![1; n];
        for &x in v.iter().rev().skip(1) { pru[x] += 1; }

        let mut adj = vec![vec![]; n];
        for &x in &v[..n-2] {
            let u = pru.iter().position(|&y| y == 1).unwrap();
            adj[u].push(x); adj[x].push(u);
            pru[x] -= 1; pru[u] -= 1;
        }
        let (x, y) = (pru.iter().position(|&x| x == 1).unwrap(), pru.iter().rposition(|&x| x == 1).unwrap());
        adj[x].push(y); adj[y].push(x);

        dfs(&adj, *v.last().unwrap(), n, false);
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<std::str::Split<'static, char>>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split('\n').peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
