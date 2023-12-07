// BOJ 30828 [Chef]
// Supported by GitHub Copilot

const INF: i32 = 0x3f3f3f3f;
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![-INF; 1<<27];
    for i in 0..n { dp[i<<18|i<<9|a[i]] = 1; }
    for d in 1..n {
        for i in 0..n-d {
            for k in 0..512 {
                dp[i<<18|i+d<<9|k] = dp[i<<18|i+d-1<<9|k]
                    .max(dp[i+1<<18|i+d<<9|k])
                    .max(dp[i+1<<18|i+d<<9|k^a[i]] + 1)
                    .max(dp[i<<18|i+d-1<<9|k^a[i+d]] + 1);
            }
        }
    }

    for _ in 0..next() {
        let (l, r) = (next::<usize>(), next::<usize>());
        let mut ans = 0;
        for k in 0..512 {
            ans = ans.max(dp[l-1<<18|r-1<<9|k] + k as i32);
        }
        println!("{}", ans);
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