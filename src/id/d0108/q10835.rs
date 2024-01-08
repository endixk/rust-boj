// BOJ 10835 [Card Game]
fn solve(dp: &mut Vec<Vec<i32>>, a: &[usize], b: &[usize], i: usize, j: usize, n: usize) -> i32 {
    if i == n || j == n { return 0; }
    if dp[i][j] != -1 { return dp[i][j]; }
    dp[i][j] = 0;
    if a[i] > b[j] { dp[i][j] = b[j] as i32 + solve(dp, a, b, i, j + 1, n); }
    dp[i][j] = dp[i][j].max(solve(dp, a, b, i + 1, j, n));
    dp[i][j] = dp[i][j].max(solve(dp, a, b, i + 1, j + 1, n));
    dp[i][j]
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let mut dp = vec![vec![-1; n + 1]; n + 1];
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