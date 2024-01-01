// BOJ 30975 [Retarded but Good]
const INF: usize = 0x3f3f3f3f;
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut p = (0..n).map(|_| next::<usize>() - 1).collect::<Vec<_>>();
    p.push(n);
    let edges = (0..m).map(|_|
        (next::<usize>() - 1, next::<usize>() - 1, next::<usize>())
    ).collect::<Vec<_>>();

    let mut dp = vec![vec![INF; 1<<n+1]; n+1];
    dp[n][0] = 0;
    let mut q = vec![0];
    for _ in 0..=n {
        let mut nq = vec![];
        let mut inq = vec![false; 1<<n+1];
        for &bit in &q {
            for &(u, v, w) in &edges {
                if dp[u][bit] == INF { continue; }
                if bit & 1<<v != 0 { continue; }
                if v != p[v] && bit & 1<<p[v] == 0 { continue; }
                dp[v][bit|1<<v] = dp[v][bit|1<<v].min(dp[u][bit] + w);
                if !inq[bit|1<<v] {
                    inq[bit|1<<v] = true;
                    nq.push(bit|1<<v);
                }
            }
        }
        q = nq;
    }
    if dp[n][(1<<n+1)-1] == INF { println!("-1"); }
    else { println!("{}", dp[n][(1<<n+1)-1]); }
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