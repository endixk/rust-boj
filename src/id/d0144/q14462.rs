// BOJ 14462 [Why Did the Cow Cross the Road II (Gold)]
fn solve(dp: &mut Vec<Vec<i16>>, a: &[i16], b: &[i16], i: usize, j: usize, n: usize) -> i16 {
    if i == n || j == n { return 0; }
    if dp[i][j] != -1 { return dp[i][j]; }
    let mut ret = solve(dp, a, b, i+1, j, n);
    let mut k = j;
    while k < n && (b[k] - a[i]).abs() > 4 { k += 1; }
    if k < n { ret = ret.max(solve(dp, a, b, i+1, k+1, n) + 1); }
    dp[i][j] = ret;
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<i16>()).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i16>()).collect::<Vec<_>>();
    let mut dp = vec![vec![-1; n]; n];
    println!("{}", solve(&mut dp, &a, &b, 0, 0, n));
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