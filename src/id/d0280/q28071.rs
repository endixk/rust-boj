// BOJ 28071 [Candies]
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![0; k];
    for _ in 0..m {
        let mut tp = dp.clone();
        for i in 0..k {
            for &x in &a {
                tp[(dp[i]+x)%k] = tp[(dp[i]+x)%k].max(dp[i] + x);
            }
        }
        dp = tp;
    }
    println!("{}", dp[0]);
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