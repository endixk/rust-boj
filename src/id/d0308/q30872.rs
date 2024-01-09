// BOJ 30872 [Hanyang Cherry Picking Contest]
// Supported by GitHub Copilot

fn dfs(adj: &Vec<Vec<usize>>, a: &Vec<i64>, cnt: &mut Vec<usize>, cur: usize, par: usize) -> usize {
    let mut ret = 1;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        ret += dfs(adj, a, cnt, nxt, cur);
    }
    cnt[cur] = ret;
    ret
}
fn go(adj: &Vec<Vec<usize>>, dp: &mut Vec<i64>, a: &Vec<i64>, cnt: &Vec<usize>, cur: usize, par: usize) -> i64 {
    if dp[cur] != -1 { return dp[cur]; }

    let mut ret = a[cur];
    let (mut p, mut q) = (0, 0);
    let mut v = vec![];
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        let k = go(adj, dp, a, cnt, nxt, cur);
        if cnt[nxt] % 2 == 0 {
            if k > 0 { p += k; }
            else { q += k; }
        } else {
            v.push(k);
        }
    }
    v.sort_unstable_by(|a, b| b.cmp(a));

    ret -= p;
    for i in 0..v.len() {
        if i % 2 == 0 { ret -= v[i]; }
        else { ret += v[i]; }
    }
    if v.len() % 2 == 0 { ret -= q; }
    else { ret += q; }

    dp[cur] = ret;
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u].push(v); adj[v].push(u);
    }

    let mut a = vec![0; n+1];
    for i in 1..=n { a[i] = next::<i64>(); }
    let mut cnt = vec![0; n+1];
    dfs(&adj, &a, &mut cnt, 1, 0);

    let mut dp = vec![-1; n+1];
    let r = go(&adj, &mut dp, &a, &cnt, 1, 0);
    if r > 0 { println!("Sehun"); }
    else if r < 0 { println!("Cheolmin"); }
    else { println!("Draw"); }
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