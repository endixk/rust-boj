// BOJ 30621 [Don't Say Huh?]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut t = vec![0; n+1];
    for i in 1..=n { t[i] = next::<usize>(); }
    let mut b = vec![0; n+1];
    for i in 1..=n { b[i] = next::<usize>(); }
    let mut c = vec![0; n+1];
    for i in 1..=n { c[i] = next::<usize>(); }

    let mut dp = vec![0; n+1];
    let mut m = 0;
    for i in 1..=n {
        let k = t.partition_point(|&x| x < t[i] - b[i]);
        let k = if k > 0 { k - 1 } else { 0 };
        m = m.max(dp[k] + c[i]);
        dp[i] = m;
    }
    println!("{}", dp.iter().max().unwrap());
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