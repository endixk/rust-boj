// BOJ 30976 [Cupid]
fn bip(adj: &Vec<Vec<usize>>, mat: &mut Vec<usize>, vis: &mut Vec<bool>, cur: usize) -> bool {
    for &nxt in &adj[cur] {
        if vis[nxt] { continue; }
        vis[nxt] = true;
        if mat[nxt] == 0 || bip(adj, mat, vis, mat[nxt]) {
            mat[nxt] = cur;
            return true;
        }
    }
    false
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let g = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let b = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
    let l = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let u = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();

    let mut adj = vec![vec![]; n+1];
    for i in 0..n {
        for j in 0..m {
            if g[i] > u[j] && b[j] < l[i] {
                adj[i+1].push(j+1);
            }
        }
    }

    let mut mat = vec![0; m+1];
    let mut ans = 0;
    for i in 1..=n {
        let mut vis = vec![false; m+1];
        if bip(&adj, &mut mat, &mut vis, i) {
            ans += 1;
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