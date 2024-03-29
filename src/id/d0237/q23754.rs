// BOJ 23754 [D2D Sales (Hard)]
// Supported by GitHub Copilot

pub fn main() { read();
    const INF: usize = 999;
    let (n, m, x, y) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n];
    let mut ind = vec![0; n];
    for _ in 0..m {
        let (u, v) = (next::<usize>()-1, next::<usize>()-1);
        adj[u].push(v);
        ind[v] += 1;
    }

    let a = (0..n).map(|_| (next::<usize>(), next::<usize>())).collect::<Vec<_>>();

    let (mut ans, mut cnt) = (0, 0);
    let mut dp = vec![vec![INF; y+1]; x+1];
    dp[0][0] = 0;
    let mut nxt = ind.iter().enumerate().filter(|(_, &x)| x == 0).map(|(i, _)| i).collect::<Vec<_>>();
    while !nxt.is_empty() {
        let mut v = vec![];
        for &i in &nxt {
            cnt += 1;
            let mut tp = dp.clone();
            for p in 0..=x {
                for q in 0..=y {
                    if dp[p][q] == INF { continue; }
                    let (np, nq) = (if p + a[i].0 > x { x } else { p + a[i].0 }, if q + a[i].1 > y { y } else { q + a[i].1 });
                    if tp[np][nq] > dp[p][q] + 1 {
                        tp[np][nq] = dp[p][q] + 1;
                        if np == x && nq == y { ans = i; }
                    }
                }
            }
            dp = tp;
            for &j in &adj[i] {
                ind[j] -= 1;
                if ind[j] == 0 { v.push(j); }
            }
        }
        v.sort_unstable();
        nxt = v;
    }

    if dp[x][y] == INF || cnt < n { println!("-1"); }
    else { println!("{}\n{}", dp[x][y], ans + 1); }
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