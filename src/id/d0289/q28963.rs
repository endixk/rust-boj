// BOJ 28963 [Pursuit]
// Supported by GitHub Copilot

#[inline] fn get(x: &[i64], s: usize, e: usize) -> Option<i64> {
    if e - s > 0 && x[s] == 0 { None }
    else {
        let mut ret = 0;
        for i in s..=e {
            ret = ret * 10 + x[i];
        }
        Some(ret)
    }
}
pub fn main() { read();
    for _ in 0..next() {
        let (x, l, r, k) = (next::<String>(), next::<i64>(), next::<i64>(), next::<usize>());
        let x = x.chars().map(|c| c.to_digit(10).unwrap() as i64).collect::<Vec<_>>();
        let n = x.len();

        let (src, snk) = (1022, 1023);
        let mut adj = vec![vec![]; 1024];
        for e in 0..n {
            if let Some(_) = get(&x, 0, e) { adj[src].push(e); }
        }
        for s in 0..n {
            if let Some(_) = get(&x, s, n-1) { adj[s<<5|n-1].push(snk); }
        }

        for s1 in 0..n { for e1 in s1..n {
            if let Some(p) = get(&x, s1, e1) {
                let s2 = e1+1;
                for e2 in s2..n {
                    if let Some(q) = get(&x, s2, e2) {
                        if l <= (p - q).abs() && (p - q).abs() <= r {
                            adj[s1<<5|e1].push(s2<<5|e2);
                        }
                    }
                }
            }
        }}

        let mut dp = vec![0; 1024];
        dp[src] = 1;
        for _ in 0..=k {
            let mut tp = vec![0; 1024];
            for u in 0..1024 {
                for &v in &adj[u] {
                    tp[v] += dp[u];
                }
            }
            dp = tp;
        }
        println!("{}", dp[snk]);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}