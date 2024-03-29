// BOJ 31413 [Enlistment]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<u32>());
    let x = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();
    let (a, d) = (next::<u32>(), next::<usize>());

    let mut dp = vec![vec![0; n+1]; n+1];
    for i in 0..n {
        for j in 0..n {
            dp[i][j+1] = dp[i][j+1].max(dp[i][j] + x[j]);
            dp[i+1][(j+d).min(n)] = dp[i+1][(j+d).min(n)].max(dp[i][j] + a);
        }
    }

    for i in 0..=n {
        if dp[i][n] >= m {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
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