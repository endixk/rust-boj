// BOJ 6547 [Fold]
fn rev_comp(p: &[bool], q: &[bool]) -> bool {
    p.iter().zip(q.iter().rev()).all(|(&a, &b)| a ^ b)
}
fn solve(dp: &mut Vec<Vec<u8>>, a: &[bool], i: usize, j: usize) -> u8 {
    if i == j { return 1; }
    if dp[i][j] != 0 { return dp[i][j]; }
    let mut ret = solve(dp, a, i+1, j).min(solve(dp, a, i, j-1)) + 1;
    for k in 1..=j-i>>1 {
        if rev_comp(&a[i..i+k], &a[i+k+1..i+2*k+1]) {
            ret = ret.min(solve(dp, a, i+k+1, j) + 1);
        }
        if rev_comp(&a[j-k+1..=j], &a[j-2*k..j-k]) {
            ret = ret.min(solve(dp, a, i, j-k-1) + 1);
        }
    }
    dp[i][j] = ret;
    ret
}
pub fn main() { read();
    while peek() {
        let a = next::<String>().chars().map(|c| c == 'A').collect::<Vec<_>>();
        let mut dp = vec![vec![0; a.len()]; a.len()];
        println!("{}", solve(&mut dp, &a, 0, a.len()-1));
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
