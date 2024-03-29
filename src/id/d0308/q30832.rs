// BOJ 30832 [Tree Reconstruction]
// Supported by GitHub Copilot

fn dfs(adj: &Vec<Vec<usize>>, par: &mut Vec<usize>, dep: &mut Vec<Vec<usize>>,
       c: usize, p: usize, d: usize) {
    par[c] = p;
    dep[d].push(c);
    for &nxt in &adj[c] {
        if nxt == p { continue; }
        dfs(adj, par, dep, nxt, c, d+1);
    }
}
pub fn main() { read();
    let n = next::<usize>();

    let mut adj1 = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj1[u].push(v); adj1[v].push(u);
    }
    let mut par1 = vec![0; n+1];
    let mut dep1 = vec![vec![]; n+1];
    dfs(&adj1, &mut par1, &mut dep1, 1, 0, 0);

    let mut adj2 = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj2[u].push(v); adj2[v].push(u);
    }
    let mut par2 = vec![0; n+1];
    let mut dep2 = vec![vec![]; n+1];
    dfs(&adj2, &mut par2, &mut dep2, 1, 0, 0);

    let mut st1 = vec![];
    for d in 2..=n {
        for &u in &dep1[d] {
            st1.push((par1[u], u));
        }
    }
    let mut st2 = vec![];
    for d in 2..=n {
        for &u in &dep2[d] {
            st2.push((par2[u], u));
        }
    }

    println!("{}", st1.len() + st2.len());
    for (u, v) in st1 {
        println!("{} {} 1", v, u);
    }
    while let Some((u, v)) = st2.pop() {
        println!("{} 1 {}", v, u);
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