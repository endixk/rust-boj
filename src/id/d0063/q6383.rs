// BOJ 6383 [SPF]
fn dfs(u: usize, adj: &Vec<Vec<usize>>, vis: &mut Vec<bool>) {
    vis[u] = true;
    for &v in &adj[u] {
        if !vis[v] { dfs(v, adj, vis) }
    }
}
pub fn main() { read();
    for tc in 1.. {
        let u = next::<usize>();
        if u == 0 { break; }

        let mut adj = vec![vec![]; 1000];
        let v = next::<usize>()-1;
        adj[u-1].push(v); adj[v].push(u-1);
        loop {
            let u = next::<usize>();
            if u == 0 { break; }
            let v = next::<usize>()-1;
            adj[u-1].push(v); adj[v].push(u-1);
        }

        println!("Network #{}", tc);
        let mut flag = false;
        for i in 0..1000 {
            let mut vis = vec![false; 1000];
            vis[i] = true;
            let mut cnt = 0;
            for j in 0..1000 {
                if adj[j].len() == 0 { continue; }
                if !vis[j] {
                    dfs(j, &adj, &mut vis);
                    cnt += 1;
                }
            }

            if cnt > 1 {
                flag = true;
                println!("  SPF node {} leaves {} subnets", i+1, cnt);
            }
        }
        if !flag { println!("  No SPF nodes") }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}