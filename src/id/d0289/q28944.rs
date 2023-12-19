// BOJ 28944 [Secret Room]
// Supported by GitHub Copilot

fn lis(a: &[usize], n: usize) -> usize {
    let mut dp = vec![a[0]];
    for i in 1..n {
        if dp.last().unwrap() < &a[i] {
            dp.push(a[i]);
        } else {
            let idx = dp.binary_search(&a[i]).unwrap_or_else(|x| x);
            dp[idx] = a[i];
        }
    }
    dp.len()
}
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|i| i + next::<usize>()).collect::<Vec<_>>();
    println!("{}", lis(&a, n));
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