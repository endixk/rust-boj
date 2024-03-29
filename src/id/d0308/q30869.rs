// BOJ 30869 [Waits Faster]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (s, e) = (next::<usize>() - 1, next::<usize>() - 1);
        let (t, g) = (next::<usize>(), next::<usize>());
        adj[s].push((e, t, g));
    }

    const INF: usize = 0x3f3f3f3f;
    let mut dp = vec![INF; n];
    dp[0] = 0;
    let mut ans = INF;
    for x in 0..=k {
        let mut v = (0..n).map(|i| (dp[i], i)).filter(|(d, _)| *d < INF).collect::<Vec<_>>();
        if v.is_empty() { break; }
        v.sort_unstable();
        let mut q = std::collections::VecDeque::new();
        let mut inq = vec![false; n];
        for (d, i) in v { q.push_back((d, i)); inq[i] = true; }

        let mut tp = vec![INF; n];
        while let Some((d, i)) = q.pop_front() {
            inq[i] = false;
            for &(j, t, g) in &adj[i] {
                let lag = if d % g == 0 { 0 } else { g - d % g };
                if dp[j] > dp[i] + t + lag {
                    dp[j] = dp[i] + t + lag;
                    if !inq[j] { q.push_back((dp[j], j)); inq[j] = true; }
                }
                if x < k && tp[j] > dp[i] + t { tp[j] = dp[i] + t; }
            }
        }

        ans = ans.min(dp[n - 1]);
        dp = tp;
    }

    if ans == INF { println!("-1"); }
    else { println!("{}", ans); }
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