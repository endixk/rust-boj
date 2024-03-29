// BOJ 14267 [Company Culture 1]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m, _) = (next::<usize>(), next::<usize>(), next::<i8>());
    let mut adj = vec![vec![]; n+1];
    for i in 2..=n {
        let p = next::<usize>();
        adj[p].push(i);
    }

    let mut dp = vec![0; n+1];
    for _ in 0..m {
        let (i, w) = (next::<usize>(), next::<i64>());
        dp[i] += w;
    }

    let mut q = vec![1];
    while let Some(i) = q.pop() {
        for &j in adj[i].iter() {
            dp[j] += dp[i];
            q.push(j);
        }
    }
    for i in 1..=n { print!("{} ", dp[i]); }
    println!();
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