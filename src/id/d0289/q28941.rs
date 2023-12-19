// BOJ 28941 [Digital Mystery]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut v = vec![];
    for _ in 0..n {
        let mut x = next::<i64>();
        let mut f = 1;
        while x > 0 {
            if x % 10 != 9 { v.push((9 - x % 10) * f); }
            x /= 10; f *= 10;
        }
    }

    v.sort_unstable();
    let mut ans = 0;
    for _ in 0..k {
        if v.is_empty() { break; }
        ans += v.pop().unwrap();
    }
    println!("{}", ans);
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