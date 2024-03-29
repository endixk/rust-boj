// BOJ 30703 [Temperature Adjustment]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();
    let b = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();

    let x = next::<i32>();
    if (a[0] - b[0]).abs() % x != 0 { println!("-1"); return; }
    let mut ans = (a[0] - b[0]).abs() / x;
    for i in 1..n {
        let x = next::<i32>();
        if (a[i] - b[i]).abs() % x != 0 { println!("-1"); return; }
        let k = (a[i] - b[i]).abs() / x;
        if ans % 2 != k % 2 { println!("-1"); return; }
        ans = ans.max(k);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}